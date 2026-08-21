use crate::Operation::ListFiles;
use crate::{Error, FilePath, FolderPath, StoragePath};
use std::fs::DirEntry;
use std::io::ErrorKind::NotFound;
use std::path::Path;

/// A local list-files operation.
pub struct LocalListFilesOp {
    root: FolderPath,
    sorted: bool,
    stack: Vec<Vec<DirEntry>>,
}

impl LocalListFilesOp {
    //! Construction

    /// Creates a local list-files operation from the `root` folder.
    pub fn new(root: FolderPath, sorted: bool) -> Result<Self, Error> {
        let entries: Vec<DirEntry> = Self::read_dir(&root, root.path().path(), sorted)?;
        Ok(Self {
            root,
            sorted,
            stack: vec![entries],
        })
    }
}

impl LocalListFilesOp {
    //! Read Directory

    /// Reads the entries in the `directory`.
    ///
    /// If `sorted` the entries will be returned in reverse lexicographical order of their full
    /// paths, so popping from the end emits files in order. Directory names are compared with a
    /// trailing file-separator since that is how they appear in the full paths beneath them.
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
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            let mut buffer: [u8; 4] = [0u8; 4];
                            key.extend_from_slice(separator.encode_utf8(&mut buffer).as_bytes());
                        }
                        std::cmp::Reverse(key)
                    })
                }
                Ok(result)
            }
            Err(e) => {
                if e.kind() == NotFound {
                    return Ok(Vec::default());
                }
                Err(error(root, e))
            }
        }
    }
}

impl LocalListFilesOp {
    //! Conversions

    /// Creates a file path from the `entry`.
    fn file_from_entry(root: &FolderPath, entry: &DirEntry) -> Result<FilePath, Error> {
        let path_buf: std::path::PathBuf = entry.path();
        let path: &str = path_buf.to_str().ok_or_else(|| {
            Error::from_message(root.clone(), ListFiles, "the file path is not UTF-8")
        })?;
        let relative: &str = path.strip_prefix(root.as_str()).ok_or_else(|| {
            Error::from_message(
                root.clone(),
                ListFiles,
                format!("the file path is not in the original folder: {}", root),
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

impl LocalListFilesOp {
    fn next_inner(&mut self) -> Option<Result<FilePath, Error>> {
        loop {
            let entries: &mut Vec<DirEntry> = self.stack.last_mut()?;
            let Some(entry) = entries.pop() else {
                self.stack.pop();
                continue;
            };
            match entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        return Some(Self::file_from_entry(&self.root, &entry));
                    } else if metadata.is_dir() {
                        match Self::read_dir(&self.root, entry.path(), self.sorted) {
                            Ok(entries) => {
                                self.stack.push(entries);
                                continue;
                            }
                            Err(error) => return Some(Err(error)),
                        }
                    } else if metadata.is_symlink() {
                        continue;
                    } else {
                        return Some(Err(Error::from_message(
                            self.root.clone(),
                            ListFiles,
                            format!("unknown entry type: {:?}", entry),
                        )));
                    }
                }
                Err(e) => {
                    return Some(Err(Error::from_message(
                        self.root.clone(),
                        ListFiles,
                        format!("error reading metadata: {}", e),
                    )))
                }
            }
        }
    }
}
