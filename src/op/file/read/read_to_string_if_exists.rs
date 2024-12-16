use std::io;
use std::io::ErrorKind::Other;

use crate::Operation::Read;
use crate::{Error, FilePath};

impl FilePath {
    //! Read to String If Exists

    /// Reads the file to the `target` `String` if it exists.
    ///
    /// Returns `Ok(Some(file_content_len))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_string_if_exists(&self, target: &mut String) -> Result<Option<usize>, Error> {
        let mut target: &mut Vec<u8> = unsafe { target.as_mut_vec() };
        let original_len: usize = target.len();
        match self.read_to_vec_if_exists(&mut target) {
            Ok(file_content_len) => {
                if let Some(file_content_len) = file_content_len {
                    debug_assert_eq!(target.len(), original_len + file_content_len);
                    let slice: &[u8] = &target[original_len..];
                    match std::str::from_utf8(slice) {
                        Ok(_) => Ok(Some(file_content_len)),
                        Err(e) => {
                            target.truncate(original_len);
                            Err(Error::from_cause(
                                self.clone(),
                                Read,
                                io::Error::new(Other, e),
                            ))
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                target.truncate(original_len);
                Err(e)
            }
        }
    }
}
