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
        if let Some(_) = crate::R2Path::from(self.path()) {
            for file in self.list_files_as_vec()?.drain(..) {
                file.delete()?;
            }
            return Ok(());
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
