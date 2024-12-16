use std::io::ErrorKind::NotFound;
use std::io::Read;

use crate::path::LocalPath;
use crate::{Error, Operation};

impl<'a> LocalPath<'a> {
    //! Read

    //! See `FilePath::read_to_vec_if_exists
    pub fn read_to_vec_if_exists(&self, mut target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        debug_assert!(self.path.is_file());

        match std::fs::File::open(self.path.as_str()) {
            Ok(mut file) => {
                let start: usize = target.len();
                let read: usize = file.read_to_end(&mut target).map_err(|error| {
                    Error::from_cause(self.path.clone(), Operation::Read, error)
                })?;
                debug_assert_eq!(target.len(), start + read);
                Ok(Some(read))
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(None)
                } else {
                    Err(Error::from_cause(self.path.clone(), Operation::Read, error))
                }
            }
        }
    }
}
