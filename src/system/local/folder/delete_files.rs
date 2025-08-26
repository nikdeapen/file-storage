use crate::system::LocalPath;
use crate::Error;
use crate::Operation::DeleteFiles;

impl<'a> LocalPath<'a> {
    //! Delete

    /// See `FolderPath::delete_files`.
    pub fn delete_files(&self) -> Result<(), Error> {
        debug_assert!(self.path.is_folder());

        let path: &str = self.path.as_str();
        if std::fs::exists(path)
            .map_err(|e| Error::from_source(self.path.clone(), DeleteFiles, e))?
        {
            std::fs::remove_dir_all(path)
                .map_err(|e| Error::from_source(self.path.clone(), DeleteFiles, e))
        } else {
            Ok(())
        }
    }
}
