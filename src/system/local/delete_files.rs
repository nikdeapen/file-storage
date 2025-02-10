use crate::system::LocalPath;
use crate::Error;
use crate::Operation::DeleteFiles;

impl<'a> LocalPath<'a> {
    //! Delete

    /// See `FolderPath::delete_files`.
    pub fn delete_files(&self) -> Result<(), Error> {
        debug_assert!(self.path.is_folder());

        std::fs::remove_dir_all(self.path.as_str())
            .map_err(|e| Error::from_cause(self.path.clone(), DeleteFiles, e))
    }
}
