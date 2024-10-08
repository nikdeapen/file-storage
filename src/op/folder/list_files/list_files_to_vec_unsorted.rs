use std::fs::DirEntry;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

use crate::Operation::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, FolderPath, StoragePath};

impl FolderPath {
    //! List Files to Vec Unsorted

    /// Lists the files to the `target` vec.
    ///
    /// Returns `Ok(file_count)`.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        if self.is_local_path() {
            self.list_files_to_vec_unsorted_local(target)
        } else {
            Err(Error::new(self, ListFiles, UnknownFileSystem))
        }
    }
}

impl FolderPath {
    //! List Files To Vec Unsorted - Local

    fn list_files_to_vec_unsorted_local(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        debug_assert!(self.is_local_path());

        let original_len: usize = target.len();
        let mut folders: Vec<PathBuf> = Vec::with_capacity(8);
        folders.push(PathBuf::from(self.as_str()));

        while let Some(current_folder) = folders.pop() {
            match std::fs::read_dir(current_folder) {
                Ok(mut read_dir) => {
                    while let Some(next) = read_dir.next() {
                        match next {
                            Ok(dir_entry) => match dir_entry.file_type() {
                                Ok(file_type) => {
                                    if file_type.is_file() {
                                        target.push(self.file_from_dir_entry(dir_entry)?);
                                    } else if file_type.is_dir() {
                                        folders.push(dir_entry.path());
                                    } else if file_type.is_symlink() {
                                        return Err(Error::from_message(
                                            self,
                                            ListFiles,
                                            "symlinks are not yet supported",
                                        ));
                                    }
                                }
                                Err(error) => {
                                    return Err(Error::from_error(self, ListFiles, error))
                                }
                            },
                            Err(error) => return Err(Error::from_error(self, ListFiles, error)),
                        }
                    }
                }
                Err(error) => {
                    if error.kind() == NotFound
                        && folders.is_empty()
                        && target.len() == original_len
                    {
                        break;
                    } else {
                        return Err(Error::from_error(self, ListFiles, error));
                    }
                }
            }
        }

        Ok(target.len() - original_len)
    }

    fn file_from_dir_entry(&self, dir_entry: DirEntry) -> Result<FilePath, Error> {
        debug_assert!(dir_entry.file_type().unwrap().is_file());

        if let Some(path) = dir_entry.path().to_str() {
            if path.starts_with(self.as_str()) {
                let extension: &str = &path[self.len()..];
                let file_path: StoragePath = self.clone_append(extension);
                if let Some(file_path) = file_path.to_file() {
                    Ok(file_path)
                } else {
                    Err(Error::from_message(
                        self,
                        ListFiles,
                        format!("local file not a file path: {}", path),
                    ))
                }
            } else {
                Err(Error::from_message(
                    self,
                    ListFiles,
                    format!("the file path is not in the original folder: {}", self),
                ))
            }
        } else {
            Err(Error::from_message(
                self,
                ListFiles,
                "the file path is not UTF-8",
            ))
        }
    }
}
