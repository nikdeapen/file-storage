use std::io::ErrorKind::AlreadyExists;

use crate::local::{is_local_path, LocalFileWrite};
use crate::op::{FileWrite, FileWriteInner};
use crate::Operation;
use crate::{Error, FilePath};

pub fn write_if_not_exists(file: &FilePath) -> Result<Option<FileWrite>, Error> {
    debug_assert!(is_local_path(file));

    if let Some(parent) = std::path::Path::new(file).parent() {
        match std::fs::create_dir_all(parent) {
            Ok(()) => {}
            Err(error) => return Err(Error::from_error(file, Operation::Write, error)),
        }
    }
    match std::fs::File::create_new(file) {
        Ok(file) => Ok(Some(FileWrite {
            inner: FileWriteInner::Local(LocalFileWrite { file }),
        })),
        Err(error) => {
            if error.kind() == AlreadyExists {
                Ok(None)
            } else {
                Err(Error::from_error(file, Operation::Write, error))
            }
        }
    }
}
