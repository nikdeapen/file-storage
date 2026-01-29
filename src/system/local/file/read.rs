use crate::system::{LocalPath, LocalReadOp};
use crate::{Error, Operation};
use std::io::ErrorKind::NotFound;
use std::io::Read;

impl<'a> LocalPath<'a> {
    //! Read

    /// See `FilePath::read_if_exists`.
    pub fn read_if_exists(&self) -> Result<Option<LocalReadOp>, Error> {
        match std::fs::File::open(self.path.as_str()) {
            Ok(file) => Ok(Some(LocalReadOp { file })),
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
        match std::fs::File::open(self.path.as_str()) {
            Ok(mut file) => {
                let file_size: u64 = file
                    .metadata()
                    .map_err(|e| Error::from_source(self.path.clone(), Operation::Read, e))?
                    .len();
                // todo -- assumes 64 bit machines
                target.reserve(file_size as usize);
                let read: usize = file
                    .read_to_end(target)
                    .map_err(|e| Error::from_source(self.path.clone(), Operation::Read, e))?;
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
