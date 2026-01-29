use crate::system::{LocalPath, LocalWriteOp};
use crate::{Error, Operation};
use std::io::ErrorKind::AlreadyExists;

impl<'a> LocalPath<'a> {
    //! Write

    /// Prepares a write operation.
    fn prepare(&self) -> Result<(), Error> {
        if let Some(parent) = std::path::Path::new(self.path).parent() {
            match std::fs::create_dir_all(parent) {
                Ok(()) => Ok(()),
                Err(error) => Err(Error::from_source(
                    self.path.clone(),
                    Operation::Write,
                    error,
                )),
            }
        } else {
            Ok(())
        }
    }

    /// See `FilePath::write_if_not_exists`.
    pub fn write_if_not_exists(&self) -> Result<Option<LocalWriteOp>, Error> {
        self.prepare()?;

        match std::fs::File::create_new(self.path) {
            Ok(file) => Ok(Some(LocalWriteOp { file })),
            Err(error) => {
                if error.kind() == AlreadyExists {
                    Ok(None)
                } else {
                    Err(Error::from_source(
                        self.path.clone(),
                        Operation::Write,
                        error,
                    ))
                }
            }
        }
    }
}
