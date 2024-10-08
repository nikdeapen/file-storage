use std::io::ErrorKind::NotFound;
use std::io::Read as IORead;

use crate::Operation::Read;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Read To Vec If Exists

    /// Reads the file into the `target` vec.
    ///
    /// Returns `Ok(Some(count))` with the number of bytes read.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        if self.is_local_path() {
            self.read_to_vec_if_exists_local(target)
        } else {
            Err(Error::new(self, Read, UnknownFileSystem))
        }
    }
}

impl FilePath {
    //! Read To Vec If Exists - Local

    pub fn read_to_vec_if_exists_local(
        &self,
        mut target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        debug_assert!(self.is_local_path());

        match std::fs::File::open(self.as_str()) {
            Ok(mut file) => {
                let start: usize = target.len();
                let read: usize = file
                    .read_to_end(&mut target)
                    .map_err(|error| Error::from_error(self, Read, error))?;
                debug_assert_eq!(target.len(), start + read);
                Ok(Some(read))
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(None)
                } else {
                    Err(Error::from_error(self, Read, error))
                }
            }
        }
    }
}
