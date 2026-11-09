use crate::system::local::is_not_found;
use crate::system::LocalPath;
use crate::Error;
use crate::Operation::Delete;
use std::io;

impl<'a> LocalPath<'a> {
    //! Delete

    /// See `FilePath::delete_if_exists`.
    pub(crate) fn delete_if_exists(self) -> Result<bool, Error> {
        let error: io::Error = match std::fs::remove_file(self.std_path()) {
            Ok(()) => return Ok(true),
            Err(error) if is_not_found(&error) => return Ok(false),
            Err(error) => error,
        };
        match std::fs::symlink_metadata(self.std_path()).map(|metadata| metadata.file_type()) {
            Ok(file_type) if file_type.is_dir() => {
                self.delete_empty_folder()?;
                Ok(false)
            }
            Ok(file_type) if self.is_folder_link(file_type) => {
                self.delete_folder_link()?;
                Ok(true)
            }
            _ => Err(Error::from_source(self.path.clone(), Delete, error)),
        }
    }

    /// Deletes the folder at the path if it is empty.
    ///
    /// # Note
    /// - A folder with anything in it is an error.
    fn delete_empty_folder(self) -> Result<(), Error> {
        match std::fs::remove_dir(self.std_path()) {
            Ok(()) => Ok(()),
            Err(error) if is_not_found(&error) => Ok(()),
            Err(error) => Err(Error::from_source(self.path.clone(), Delete, error)),
        }
    }

    /// Deletes the Windows symlink to a folder at the path.
    ///
    /// # Note
    /// - Windows deletes a folder link with `remove_dir`. Only the link is deleted, never its target.
    fn delete_folder_link(self) -> Result<(), Error> {
        std::fs::remove_dir(self.std_path())
            .map_err(|error| Error::from_source(self.path.clone(), Delete, error))
    }
}
