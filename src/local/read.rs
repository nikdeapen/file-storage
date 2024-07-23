use std::io::ErrorKind::NotFound;

use crate::local::{is_local_path, LocalFileRead};
use crate::op::{FileRead, FileReadInner};
use crate::Operation::Read;
use crate::{Error, FilePath};

pub fn read_if_exists(file: &FilePath) -> Result<Option<FileRead>, Error> {
    debug_assert!(is_local_path(file));

    match std::fs::File::open(file) {
        Ok(file) => Ok(Some(FileRead {
            inner: FileReadInner::Local(LocalFileRead { file }),
        })),
        Err(error) => {
            if error.kind() == NotFound {
                Ok(None)
            } else {
                Err(Error::from_error(file, Read, error))
            }
        }
    }
}
