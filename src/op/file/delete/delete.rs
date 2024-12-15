use crate::Operation::Delete;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Delete

    /// Deletes the file.
    ///
    /// Returns `Ok(())` if the file was deleted or if the file did not exist.
    pub fn delete(&self) -> Result<(), Error> {
        if self.is_local_path() {
            return self.delete_if_exists().map(|_| ());
        }

        Err(Error::new(self.clone(), Delete, UnknownFileSystem))
    }
}
