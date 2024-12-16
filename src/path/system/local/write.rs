use std::io::ErrorKind::AlreadyExists;
use std::io::Write;

use crate::path::LocalPath;
use crate::{Error, Operation};

impl<'a> LocalPath<'a> {
    //! Local

    /// See `FilePath::write_data_if_not_exists`.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        debug_assert!(self.path.is_file());

        if let Some(parent) = std::path::Path::new(self.path).parent() {
            match std::fs::create_dir_all(parent) {
                Ok(()) => {}
                Err(error) => {
                    return Err(Error::from_cause(
                        self.path.clone(),
                        Operation::Write,
                        error,
                    ))
                }
            }
        }
        match std::fs::File::create_new(self.path) {
            Ok(mut file) => {
                file.write_all(data.as_ref())
                    .map_err(|e| Error::from_cause(self.path.clone(), Operation::Write, e))?;
                file.sync_all()
                    .map_err(|e| Error::from_cause(self.path.clone(), Operation::Write, e))?;
                Ok(true)
            }
            Err(error) => {
                if error.kind() == AlreadyExists {
                    Ok(false)
                } else {
                    Err(Error::from_cause(
                        self.path.clone(),
                        Operation::Write,
                        error,
                    ))
                }
            }
        }
    }
}
