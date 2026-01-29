use crate::Operation::ListFiles;
use crate::{Error, FilePath, FolderPath, StoragePath};
use std::fs::DirEntry;
use std::io::ErrorKind::NotFound;
use std::path::Path;

/// A local list-files operation.
pub struct LocalListFilesOp {
    root: FolderPath,
    sorted: bool,
    recursive: RecursiveOp,
}

impl LocalListFilesOp {
    //! Construction

    /// Creates a local list-files operation from the `root` folder.
    pub fn from(root: FolderPath, sorted: bool) -> Result<Self, Error> {
        let entries: Vec<DirEntry> = Self::read_dir(&root, root.path().path(), sorted)?;
        Ok(Self {
            root,
            sorted,
            recursive: RecursiveOp {
                recursive: None,
                entries,
            },
        })
    }
}

impl LocalListFilesOp {
    //! Read Directory

    /// Reads the entries in the `directory`.
    ///
    /// If `sorted` the entries will be returned in reverse sorted order.
    fn read_dir<P>(root: &FolderPath, directory: P, sorted: bool) -> Result<Vec<DirEntry>, Error>
    where
        P: AsRef<Path>,
    {
        fn error<P>(root: &FolderPath, dir: P, e: std::io::Error) -> Error
        where
            P: AsRef<Path>,
        {
            Error::from_message(
                root.clone(),
                ListFiles,
                format!(
                    "error reading local directory: dir={:?} error={}",
                    dir.as_ref(),
                    e
                ),
            )
        }
        match std::fs::read_dir(directory.as_ref()) {
            Ok(read_dir) => {
                let mut result: Vec<DirEntry> = Vec::default();
                for next in read_dir {
                    match next {
                        Ok(entry) => result.push(entry),
                        Err(e) => return Err(error(root, directory, e)),
                    }
                }
                if sorted {
                    result.sort_by_key(|b| std::cmp::Reverse(b.file_name()))
                }
                Ok(result)
            }
            Err(e) => {
                if e.kind() == NotFound {
                    return Ok(Vec::default());
                }
                Err(error(root, directory, e))
            }
        }
    }
}

impl Iterator for LocalListFilesOp {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.recursive.next(&self.root, self.sorted)
    }
}

struct RecursiveOp {
    recursive: Option<Box<RecursiveOp>>,
    entries: Vec<DirEntry>,
}

impl RecursiveOp {
    //! Next

    /// Gets the next file path.
    ///
    /// Returns `None` when there are no remaining file paths.
    fn next(&mut self, root: &FolderPath, sorted: bool) -> Option<Result<FilePath, Error>> {
        match self.next_no_reset(root, sorted) {
            None => None,
            Some(result) => {
                if result.is_err() {
                    self.recursive = None;
                    self.entries = Vec::default();
                }
                Some(result)
            }
        }
    }

    fn next_no_reset(
        &mut self,
        root: &FolderPath,
        sorted: bool,
    ) -> Option<Result<FilePath, Error>> {
        loop {
            if let Some(recursive) = &mut self.recursive {
                if let Some(next) = recursive.next(root, sorted) {
                    return Some(next);
                } else {
                    self.recursive = None;
                }
            } else if let Some(entry) = self.entries.pop() {
                match entry.metadata() {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            return Some(Self::file_from_entry(root, &entry));
                        } else if metadata.is_dir() {
                            match LocalListFilesOp::read_dir(root, entry.path(), sorted) {
                                Ok(entries) => {
                                    let recursive: RecursiveOp = RecursiveOp {
                                        recursive: None,
                                        entries,
                                    };
                                    self.recursive = Some(Box::new(recursive));
                                }
                                Err(error) => return Some(Err(error)),
                            }
                        } else if metadata.is_symlink() {
                            // todo -- symlink can result in forever loop, just treat as file?
                            return Some(Err(Error::from_message(
                                root.clone(),
                                ListFiles,
                                "symlink not yet supported",
                            )));
                        } else {
                            return Some(Err(Error::from_message(
                                root.clone(),
                                ListFiles,
                                format!("unknown entry type: {:?}", entry),
                            )));
                        }
                    }
                    Err(e) => {
                        return Some(Err(Error::from_message(
                            root.clone(),
                            ListFiles,
                            format!("error reading metadata: {}", e),
                        )))
                    }
                }
            } else if self.recursive.is_none() && self.entries.is_empty() {
                return None;
            }
        }
    }
}

impl RecursiveOp {
    //! Conversions

    /// Creates a file path from the `entry`.
    fn file_from_entry(root: &FolderPath, entry: &DirEntry) -> Result<FilePath, Error> {
        debug_assert!(entry.file_type().unwrap().is_file());

        if let Some(path) = entry.path().to_str() {
            if path.starts_with(root.as_str()) {
                let extension: &str = &path[root.len()..];
                let file_path: StoragePath = root.clone_append(extension);
                if let Ok(file_path) = file_path.to_file() {
                    Ok(file_path)
                } else {
                    Err(Error::from_message(
                        root.clone(),
                        ListFiles,
                        format!("local file not a file path: {}", path),
                    ))
                }
            } else {
                Err(Error::from_message(
                    root.clone(),
                    ListFiles,
                    format!("the file path is not in the original folder: {}", root),
                ))
            }
        } else {
            Err(Error::from_message(
                root.clone(),
                ListFiles,
                "the file path is not UTF-8",
            ))
        }
    }
}
