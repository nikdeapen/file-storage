use crate::system::R2Path;
use crate::Error;

impl<'a> R2Path<'a> {
    //! Delete

    /// See `FolderPath::delete_files`.
    pub fn delete_files(&self) -> Result<(), Error> {
        debug_assert!(self.path.is_folder());

        // todo -- batch deletes
        for file in self.path.clone().to_folder().unwrap().list_files_as_vec()? {
            file.delete()?;
        }

        Ok(())
    }
}
