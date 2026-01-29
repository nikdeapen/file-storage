use crate::op::WriteOpInner;
use crate::system::LocalPath;
use crate::Operation::Write;
use crate::Reason;
use crate::{Error, FilePath, WriteOp};

impl FilePath {
    //! Write

    /// Opens a write operation to the file.
    ///
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write(&self) -> Result<WriteOp, Error> {
        if let Some(write) = self.write_if_not_exists()? {
            Ok(write)
        } else {
            Err(Error::new(
                self.path().clone(),
                Write,
                Reason::FileAlreadyExists,
            ))
        }
    }

    /// Opens a write operation to the file if it exists.
    pub fn write_if_not_exists(&self) -> Result<Option<WriteOp>, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.write_if_not_exists().map(|op| {
                op.map(|op| WriteOp {
                    inner: WriteOpInner::Local(op),
                })
            });
        }

        #[cfg(feature = "r2")]
        if crate::R2Path::from(self.path()).is_some() {
            // todo -- r2 write operations
            return Err(Error::new(
                self.path().clone(),
                Write,
                Reason::UnsupportedOperation,
            ));
        }

        Err(Error::new(
            self.path().clone(),
            Write,
            Reason::UnknownFileSystem,
        ))
    }
}
