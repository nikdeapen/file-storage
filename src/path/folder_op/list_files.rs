use crate::op::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, FolderPath, Operation};

impl FolderPath {
    //! List

    /// Lists the files in the folder.
    pub fn list_files(&self) -> Result<ListFiles, Error> {
        if crate::local::is_local_path(self) {
            return crate::local::list(self);
        }

        Err(Error::new(self, Operation::ListFiles, UnknownFileSystem))
    }

    /// Lists the files in a folder as a `Vec<FilePath>`.
    pub fn list_files_as_vec(&self) -> Result<Vec<FilePath>, Error> {
        if crate::local::is_local_path(self) {
            return crate::local::list_as_vec(self);
        }

        Err(Error::new(self, Operation::ListFiles, UnknownFileSystem))
    }
}
