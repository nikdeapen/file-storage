use crate::op::ReadFileInner;
use crate::system::{LocalFileRead, LocalPath};
use crate::{Error, FileRead, Operation};
use std::io::ErrorKind::NotFound;
use std::io::Read;

impl<'a> LocalPath<'a> {
    //! Read

    /// See `FilePath::read_if_exists`.
    pub fn read_if_exists(&self) -> Result<Option<FileRead>, Error> {
        match std::fs::File::open(self.path.as_str()) {
            Ok(file) => Ok(Some(FileRead {
                inner: ReadFileInner::Local(LocalFileRead { file }),
            })),
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(None)
                } else {
                    Err(Error::from_source(
                        self.path.clone(),
                        Operation::Read,
                        error,
                    ))
                }
            }
        }
    }

    /// See `FilePath::read_to_vec_if_exists`.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        debug_assert!(self.path.is_file());

        match std::fs::File::open(self.path.as_str()) {
            Ok(mut file) => {
                let start: usize = target.len();
                let read: usize = file.read_to_end(target).map_err(|error| {
                    Error::from_source(self.path.clone(), Operation::Read, error)
                })?;
                debug_assert_eq!(target.len(), start + read);
                Ok(Some(read))
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(None)
                } else {
                    Err(Error::from_source(
                        self.path.clone(),
                        Operation::Read,
                        error,
                    ))
                }
            }
        }
    }
}
