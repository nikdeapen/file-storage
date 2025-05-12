use std::fs::DirEntry;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

use crate::system::LocalPath;
use crate::Operation::ListFiles;
use crate::{Error, FilePath, StoragePath};

impl<'a> LocalPath<'a> {
    //! List Files

    /// See `FolderPath::list_files_to_vec_unsorted`.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        let original_len: usize = target.len();
        let mut folders: Vec<PathBuf> = Vec::with_capacity(8);
        folders.push(PathBuf::from(self.path.as_str()));

        while let Some(current_folder) = folders.pop() {
            match std::fs::read_dir(current_folder) {
                Ok(mut read_dir) => {
                    for next in read_dir.by_ref() {
                        match next {
                            Ok(dir_entry) => match dir_entry.file_type() {
                                Ok(file_type) => {
                                    if file_type.is_file() {
                                        target.push(self.file_from_dir_entry(dir_entry)?);
                                    } else if file_type.is_dir() {
                                        folders.push(dir_entry.path());
                                    } else if file_type.is_symlink() {
                                        return Err(Error::from_message(
                                            self.path.clone(),
                                            ListFiles,
                                            "symlinks are not yet supported",
                                        ));
                                    }
                                }
                                Err(error) => {
                                    return Err(Error::from_cause(
                                        self.path.clone(),
                                        ListFiles,
                                        error,
                                    ))
                                }
                            },
                            Err(error) => {
                                return Err(Error::from_cause(self.path.clone(), ListFiles, error))
                            }
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
                        return Err(Error::from_cause(self.path.clone(), ListFiles, error));
                    }
                }
            }
        }

        Ok(target.len() - original_len)
    }

    fn file_from_dir_entry(&self, dir_entry: DirEntry) -> Result<FilePath, Error> {
        debug_assert!(dir_entry.file_type().unwrap().is_file());

        if let Some(path) = dir_entry.path().to_str() {
            if path.starts_with(self.path.as_str()) {
                let extension: &str = &path[self.path.len()..];
                let file_path: StoragePath = self.path.clone_append(extension);
                if let Ok(file_path) = file_path.to_file() {
                    Ok(file_path)
                } else {
                    Err(Error::from_message(
                        self.path.clone(),
                        ListFiles,
                        format!("local file not a file path: {}", path),
                    ))
                }
            } else {
                Err(Error::from_message(
                    self.path.clone(),
                    ListFiles,
                    format!("the file path is not in the original folder: {}", self.path),
                ))
            }
        } else {
            Err(Error::from_message(
                self.path.clone(),
                ListFiles,
                "the file path is not UTF-8",
            ))
        }
    }
}
