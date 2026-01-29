use crate::system::{LocalListFilesOp, LocalPath};
use crate::Error;

impl<'a> LocalPath<'a> {
    //! List Files

    /// See `FolderPath::list_files`.
    pub fn list_files(&self) -> Result<LocalListFilesOp, Error> {
        LocalListFilesOp::from(self.path.clone().make_folder(), true)
    }

    /// See `FolderPath::list_files_unsorted`.
    pub fn list_files_unsorted(&self) -> Result<LocalListFilesOp, Error> {
        LocalListFilesOp::from(self.path.clone().make_folder(), false)
    }
}
