use crate::Operation::Read;
use crate::Reason::{FileContentNotUtf8, FileNotFound};
use crate::{Error, FilePath};

/// A drop guard that truncates a `Vec<u8>` to its original length if not disarmed.
struct TruncateGuard<'a> {
    vec: &'a mut Vec<u8>,
    original_len: usize,
    armed: bool,
}

impl<'a> TruncateGuard<'a> {
    //! Construction

    /// Creates the [TruncateGuard].
    fn new(vec: &'a mut Vec<u8>) -> Self {
        let original_len: usize = vec.len();
        Self {
            vec,
            original_len,
            armed: true,
        }
    }
}

impl<'a> TruncateGuard<'a> {
    //! Disarm

    /// Keeps the `Vec` as it is when the guard drops.
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
    /// # Note
    /// - A missing file fails with `FileNotFound`. See `read_if_exists` for what counts as missing.
    /// - Content that is not UTF-8 fails with `FileContentNotUtf8`.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(file_content) = self.read_as_string_if_exists()? {
            Ok(file_content)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file as a `String` if it exists.
    ///
    /// # Note
    /// - It is `None` if the file does not exist. See `read_if_exists` for what counts as missing.
    /// - Content that is not UTF-8 fails with `FileContentNotUtf8`.
    pub fn read_as_string_if_exists(&self) -> Result<Option<String>, Error> {
        if let Some(file_content) = self.read_as_vec_if_exists()? {
            match String::from_utf8(file_content) {
                Ok(file_content) => Ok(Some(file_content)),
                Err(_) => Err(Error::new(self.clone(), Read, FileContentNotUtf8)),
            }
        } else {
            Ok(None)
        }
    }

    /// Reads the file to the end of the `target` `String`.
    ///
    /// # Note
    /// - It returns the number of bytes read.
    /// - A missing file fails with `FileNotFound`. See `read_if_exists` for what counts as missing.
    /// - Content that is not UTF-8 fails with `FileContentNotUtf8`.
    /// - On error the `target` is left as it was.
    pub fn read_to_string(&self, target: &mut String) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_string_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file to the end of the `target` `String` if it exists.
    ///
    /// # Note
    /// - It returns the number of bytes read. It is `None` if the file does not exist.
    /// - See `read_if_exists` for what counts as missing.
    /// - Content that is not UTF-8 fails with `FileContentNotUtf8`.
    /// - On error the `target` is left as it was.
    pub fn read_to_string_if_exists(&self, target: &mut String) -> Result<Option<usize>, Error> {
        // Safety: the guard cuts the bytes back to their original length on an error or a panic.
        // The new bytes are only kept once they are checked as UTF-8.
        let mut guard: TruncateGuard = TruncateGuard::new(unsafe { target.as_mut_vec() });
        let Some(file_content_len) = self.read_to_vec_if_exists(guard.vec)? else {
            return Ok(None);
        };
        debug_assert_eq!(guard.vec.len(), guard.original_len + file_content_len);
        if std::str::from_utf8(&guard.vec[guard.original_len..]).is_err() {
            return Err(Error::new(self.clone(), Read, FileContentNotUtf8));
        }
        guard.disarm();
        Ok(Some(file_content_len))
    }
}
