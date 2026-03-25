use crate::Operation::Read;
use crate::Reason::{FileContentNotUTF8, FileNotFound};
use crate::{Error, FilePath};

/// A drop guard that truncates a `Vec<u8>` to its original length if not disarmed.
struct TruncateGuard<'a> {
    vec: &'a mut Vec<u8>,
    original_len: usize,
    armed: bool,
}

impl<'a> TruncateGuard<'a> {
    fn new(vec: &'a mut Vec<u8>) -> Self {
        let original_len: usize = vec.len();
        Self {
            vec,
            original_len,
            armed: true,
        }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TruncateGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            self.vec.truncate(self.original_len);
        }
    }
}

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
        // Safety: we validate UTF-8 before returning. The TruncateGuard ensures the Vec is
        // truncated to its original length on error or panic-unwind, preserving String validity.
        let target: &mut Vec<u8> = unsafe { target.as_mut_vec() };
        let mut guard: TruncateGuard = TruncateGuard::new(target);
        let original_len: usize = guard.original_len;
        match self.read_to_vec_if_exists(guard.vec) {
            Ok(file_content_len) => {
                if let Some(file_content_len) = file_content_len {
                    debug_assert_eq!(guard.vec.len(), original_len + file_content_len);
                    let slice: &[u8] = &guard.vec[original_len..];
                    match std::str::from_utf8(slice) {
                        Ok(_) => {
                            guard.disarm();
                            Ok(Some(file_content_len))
                        }
                        Err(_) => Err(Error::new(self.clone(), Read, FileContentNotUTF8)),
                    }
                } else {
                    guard.disarm();
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }
}
