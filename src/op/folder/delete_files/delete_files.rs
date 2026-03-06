use crate::system::LocalPath;
use crate::Operation::Delete;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FolderPath};

impl FolderPath {
    //! Delete Files

    /// Deletes all files in the folder recursively.
    pub fn delete_files(&self) -> Result<(), Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.delete_files();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::from(self.path()) {
            return path.delete_files();
        }

        Err(Error::new(self.path().clone(), Delete, UnknownFileSystem))
    }
}
