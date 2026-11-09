use crate::system::local::is_not_found;
use crate::Operation::ListFiles;
use crate::{Error, FilePath, FolderPath, StoragePath};
use std::fs::{DirEntry, FileType, Metadata};
use std::path::{Path, PathBuf};

/// A local list-files operation.
///
/// # Note
/// - Files and folders that disappear during the walk are skipped.
/// - Symlinks are followed. A link back to a folder being listed is skipped.
/// - Entries that are neither files nor folders, such as sockets, are skipped.
pub(crate) struct LocalListFilesOp {
    root: FolderPath,
    sorted: bool,
    stack: Vec<Folder>,
}

/// A folder being listed.
struct Folder {
    /// The entries left to visit. When sorted they are in reverse order.
    entries: Vec<DirEntry>,

    /// The folder's real path, with symlinks resolved. It finds links that loop.
    real_path: PathBuf,
}

impl LocalListFilesOp {
    //! Read Directory

    /// Reads the entries in the `directory`.
    ///
    /// # Note
    /// - When `sorted`, the entries are in reverse order of their full paths. Popping from the end
    ///   gives the files in order.
    /// - A folder's name is compared with a trailing file separator. That is how it appears in the
    ///   full paths beneath it.
    fn read_dir<P>(root: &FolderPath, directory: P, sorted: bool) -> Result<Vec<DirEntry>, Error>
    where
        P: AsRef<Path>,
    {
        fn error(root: &FolderPath, e: std::io::Error) -> Error {
            Error::from_source(root.clone(), ListFiles, e)
        }
        match std::fs::read_dir(directory.as_ref()) {
            Ok(read_dir) => {
                let mut result: Vec<DirEntry> = Vec::default();
                for next in read_dir {
                    match next {
                        Ok(entry) => result.push(entry),
                        Err(e) => return Err(error(root, e)),
                    }
                }
                if sorted {
                    let separator: char = root.file_separator();
                    result.sort_by_cached_key(|entry| {
                        let mut key: Vec<u8> = entry.file_name().as_encoded_bytes().to_vec();
                        if Self::is_folder(entry) {
                            let mut buffer: [u8; 4] = [0u8; 4];
                            key.extend_from_slice(separator.encode_utf8(&mut buffer).as_bytes());
                        }
                        std::cmp::Reverse(key)
                    })
                }
                Ok(result)
            }
            Err(e) => {
                if is_not_found(&e) {
                    return Ok(Vec::default());
                }
                Err(error(root, e))
            }
        }
    }

    /// Checks if the `entry` is a folder, following a symlink.
    fn is_folder(entry: &DirEntry) -> bool {
        match entry.file_type() {
            Ok(file_type) if file_type.is_symlink() => std::fs::metadata(entry.path())
                .map(|metadata| metadata.is_dir())
                .unwrap_or(false),
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        }
    }
}

impl LocalListFilesOp {
    //! Construction

    /// Creates the [LocalListFilesOp].
    pub(crate) fn new(root: FolderPath, sorted: bool) -> Result<Self, Error> {
        let entries: Vec<DirEntry> = Self::read_dir(&root, root.path().path(), sorted)?;
        let real_path: PathBuf = match std::fs::canonicalize(root.as_str()) {
            Ok(real_path) => real_path,
            Err(e) if is_not_found(&e) => PathBuf::default(),
            Err(e) => return Err(Error::from_source(root.clone(), ListFiles, e)),
        };
        Ok(Self {
            root,
            sorted,
            stack: vec![Folder { entries, real_path }],
        })
    }
}

impl LocalListFilesOp {
    //! Walk

    /// Gets the next file in the walk.
    fn next_inner(&mut self) -> Option<Result<FilePath, Error>> {
        loop {
            let folder: &mut Folder = self.stack.last_mut()?;
            let Some(entry) = folder.entries.pop() else {
                self.stack.pop();
                continue;
            };
            let file_type: FileType = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(e) if is_not_found(&e) => continue,
                Err(e) => return Some(Err(Error::from_source(self.root.clone(), ListFiles, e))),
            };
            let is_symlink: bool = file_type.is_symlink();
            let metadata: std::io::Result<Metadata> = if is_symlink {
                std::fs::metadata(entry.path())
            } else {
                entry.metadata()
            };
            match metadata {
                Ok(metadata) if metadata.is_file() => {
                    return Some(self.file_from_entry(&entry));
                }
                Ok(metadata) if metadata.is_dir() => match self.enter_folder(&entry, is_symlink) {
                    Ok(()) => continue,
                    Err(error) => return Some(Err(error)),
                },
                // Other entries, such as sockets and FIFOs, are not files. They are skipped.
                Ok(_metadata) => continue,
                Err(e) if is_not_found(&e) => continue,
                Err(e) => return Some(Err(Error::from_source(self.root.clone(), ListFiles, e))),
            }
        }
    }

    /// Pushes the folder at the `entry` onto the walk.
    ///
    /// # Note
    /// - Only a symlink can loop. A linked folder whose real path is already being walked is
    ///   skipped.
    fn enter_folder(&mut self, entry: &DirEntry, is_symlink: bool) -> Result<(), Error> {
        let real_path: PathBuf = if is_symlink {
            match std::fs::canonicalize(entry.path()) {
                Ok(real_path) => real_path,
                Err(e) if is_not_found(&e) => return Ok(()),
                Err(e) => return Err(Error::from_source(self.root.clone(), ListFiles, e)),
            }
        } else {
            match self.stack.last() {
                Some(parent) => parent.real_path.join(entry.file_name()),
                None => return Ok(()),
            }
        };
        let loops: bool = is_symlink && self.stack.iter().any(|f| f.real_path == real_path);
        if loops {
            return Ok(());
        }
        let entries: Vec<DirEntry> = Self::read_dir(&self.root, entry.path(), self.sorted)?;
        self.stack.push(Folder { entries, real_path });
        Ok(())
    }

    /// Creates a file path from the `entry`.
    fn file_from_entry(&self, entry: &DirEntry) -> Result<FilePath, Error> {
        let root: &FolderPath = &self.root;
        let path_buf: PathBuf = entry.path();
        let path: &str = path_buf.to_str().ok_or_else(|| {
            Error::from_message(root.clone(), ListFiles, "the file path is not UTF-8")
        })?;
        let relative: &str = path.strip_prefix(root.as_str()).ok_or_else(|| {
            Error::from_message(
                root.clone(),
                ListFiles,
                format!("the file path is not in the original folder: {}", path),
            )
        })?;
        let file_path: StoragePath = root.clone_append(relative);
        file_path.to_file().map_err(|error| {
            Error::from_message(
                root.clone(),
                ListFiles,
                format!("local file not a file path: {}", error.path()),
            )
        })
    }
}

impl Iterator for LocalListFilesOp {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_inner() {
            Some(Err(error)) => {
                self.stack.clear();
                Some(Err(error))
            }
            other => other,
        }
    }
}
