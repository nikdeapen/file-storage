use crate::op::FileWriteInner;
use crate::system::{LocalFileWrite, LocalPath};
use crate::{Error, FileWrite, Operation};
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;

impl<'a> LocalPath<'a> {
    //! Local

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
    pub fn write_if_not_exists(&self) -> Result<Option<FileWrite>, Error> {
        self.prepare()?;

        match std::fs::File::create_new(self.path) {
            Ok(file) => {
                let write: FileWrite = FileWrite {
                    inner: FileWriteInner::Local(LocalFileWrite { file }),
                };
                Ok(Some(write))
            }
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

    /// See `FilePath::write_slice_if_not_exists`.
    pub fn write_slice_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        self.prepare()?;

        match std::fs::File::create_new(self.path) {
            Ok(mut file) => {
                file.write_all(data.as_ref())
                    .map_err(|e| Error::from_source(self.path.clone(), Operation::Write, e))?;
                file.sync_all()
                    .map_err(|e| Error::from_source(self.path.clone(), Operation::Write, e))?;
                Ok(true)
            }
            Err(error) => {
                if error.kind() == AlreadyExists {
                    Ok(false)
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
