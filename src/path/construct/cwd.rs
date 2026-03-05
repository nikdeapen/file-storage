use crate::{Error, FolderPath, StoragePath};
use std::io;

impl FolderPath {
    //! Current Working Directory

    /// Gets the current working directory.
    pub fn current_working_directory() -> Result<Self, io::Error> {
        let path = std::env::current_dir()?;
        match path.into_os_string().into_string() {
            Ok(path) => Ok(StoragePath::parse(path)?.make_folder()),
            Err(os) => Err(Error::path_not_utf8(os.as_ref())),
        }
    }
}
