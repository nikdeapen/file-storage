use crate::system::LocalPath;
use crate::Operation::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FolderPath};

impl FolderPath {
    //! Delete Files

    /// Deletes the files in the folder.
    ///
    /// Returns `Ok(())`.
    pub fn delete_files(&self) -> Result<(), Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.delete_files();
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::system::R2Path::from(self.path()) {
            return r2.delete_files();
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
