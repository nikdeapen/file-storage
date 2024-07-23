use std::fs::DirEntry;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

use crate::local::{is_local_path, LocalFileList};
use crate::op::{FileListInner, ListFiles};
use crate::{Error, FilePath, FolderPath, Operation, Path};

pub fn list(folder: &FolderPath) -> Result<ListFiles, Error> {
    debug_assert!(is_local_path(folder));

    let mut files: Vec<FilePath> = list_unsorted(folder)?;
    files.sort_unstable_by(|a, b| b.cmp(a));
    Ok(ListFiles {
        inner: FileListInner::Local(LocalFileList {
            files_reversed: files,
        }),
    })
}

pub fn list_as_vec(folder: &FolderPath) -> Result<Vec<FilePath>, Error> {
    debug_assert!(is_local_path(folder));

    let mut files: Vec<FilePath> = list_unsorted(folder)?;
    files.sort_unstable();
    Ok(files)
}

fn list_unsorted(folder: &FolderPath) -> Result<Vec<FilePath>, Error> {
    debug_assert!(is_local_path(folder));

    let mut files: Vec<FilePath> = Vec::with_capacity(64);
    let mut folders: Vec<PathBuf> = Vec::with_capacity(8);
    folders.push(PathBuf::from(folder));

    while let Some(current_folder) = folders.pop() {
        match std::fs::read_dir(current_folder) {
            Ok(mut read_dir) => {
                while let Some(next) = read_dir.next() {
                    match next {
                        Ok(dir_entry) => match dir_entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_file() {
                                    files.push(file_from_dir_entry(folder, dir_entry)?);
                                } else if file_type.is_dir() {
                                    folders.push(dir_entry.path());
                                } else if file_type.is_symlink() {
                                    return Err(Error::from_message(
                                        folder,
                                        Operation::ListFiles,
                                        "symlinks not yet supported",
                                    ));
                                }
                            }
                            Err(error) => {
                                return Err(Error::from_error(folder, Operation::ListFiles, error))
                            }
                        },
                        Err(error) => {
                            return Err(Error::from_error(folder, Operation::ListFiles, error))
                        }
                    }
                }
            }
            Err(error) => {
                if error.kind() == NotFound && folders.is_empty() && files.is_empty() {
                    break;
                } else {
                    return Err(Error::from_error(folder, Operation::ListFiles, error));
                }
            }
        }
    }

    Ok(files)
}

fn file_from_dir_entry(folder: &FolderPath, dir_entry: DirEntry) -> Result<FilePath, Error> {
    debug_assert!(dir_entry.file_type().unwrap().is_file());

    if let Some(path) = dir_entry.path().to_str() {
        if path.starts_with(folder.path().path()) {
            let extension: &str = &path[folder.path().path().len()..];
            let file_path: Path = folder.path().clone_append(extension);
            if let Some(file_path) = file_path.to_file() {
                Ok(file_path)
            } else {
                Err(Error::from_message(
                    folder,
                    Operation::ListFiles,
                    format!("local file not a file path: {}", path),
                ))
            }
        } else {
            Err(Error::from_message(
                folder,
                Operation::ListFiles,
                format!("file path not in the original folder: {}", path),
            ))
        }
    } else {
        Err(Error::from_message(
            folder,
            Operation::ListFiles,
            "file path not a valid string",
        ))
    }
}
