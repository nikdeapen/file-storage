use crate::Operation::Delete;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Delete No Feedback

    /// Deletes the file.
    ///
    /// Provides no feedback on if the file existed.
    pub fn delete_no_feedback(&self) -> Result<(), Error> {
        if self.is_local_path() {
            self.delete_if_exists_local().map(|_| ())
        } else {
            Err(Error::new(self, Delete, UnknownFileSystem))
        }
    }
}
