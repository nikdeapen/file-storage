use crate::op::FileList;
use crate::ErrorInfo::UnknownFileSystem;
use crate::Operation::List;
use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! List

    /// Lists the files in the folder.
    pub fn list_files(&self) -> Result<FileList, Error> {
        #[cfg(feature = "local")]
        if crate::local::is_local_path(self) {
            return crate::local::list(self);
        }

        Err(Error::new(self, List, UnknownFileSystem))
    }

    /// Lists the files in a folder as a `Vec<FilePath>`.
    pub fn list_files_as_vec(&self) -> Result<Vec<FilePath>, Error> {
        #[cfg(feature = "local")]
        if crate::local::is_local_path(self) {
            return crate::local::list_as_vec(self);
        }

        Err(Error::new(self, List, UnknownFileSystem))
    }
}
