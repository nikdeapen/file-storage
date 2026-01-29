use crate::Operation::Read;
use crate::Reason::{FileContentNotUTF8, FileNotFound};
use crate::{Error, FilePath};

impl FilePath {
    //! Read String

    /// Reads the file as a `String`.
    ///
    /// Returns `Ok(file_content)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(file_content) = self.read_as_string_if_exists()? {
            Ok(file_content)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file as a `String` if it exists.
    ///
    /// Returns `Ok(Some(file_content))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_as_string_if_exists(&self) -> Result<Option<String>, Error> {
        if let Some(file_content) = self.read_as_vec_if_exists()? {
            match String::from_utf8(file_content) {
                Ok(file_content) => Ok(Some(file_content)),
                Err(_) => Err(Error::new(self.clone(), Read, FileContentNotUTF8)),
            }
        } else {
            Ok(None)
        }
    }

    /// Reads the file to the `target` `String`.
    ///
    /// Returns `Ok(file_content_len)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_to_string(&self, target: &mut String) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_string_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file to the `target` `String` if it exists.
    ///
    /// Returns `Ok(Some(file_content_len))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_string_if_exists(&self, target: &mut String) -> Result<Option<usize>, Error> {
        // todo -- check unsafe with panics
        let target: &mut Vec<u8> = unsafe { target.as_mut_vec() };
        let original_len: usize = target.len();
        match self.read_to_vec_if_exists(target) {
            Ok(file_content_len) => {
                if let Some(file_content_len) = file_content_len {
                    debug_assert_eq!(target.len(), original_len + file_content_len);
                    let slice: &[u8] = &target[original_len..];
                    match std::str::from_utf8(slice) {
                        Ok(_) => Ok(Some(file_content_len)),
                        Err(_) => {
                            target.truncate(original_len);
                            Err(Error::new(self.clone(), Read, FileContentNotUTF8))
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
