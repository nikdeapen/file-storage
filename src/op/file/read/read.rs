use crate::op::file::read::read_op::{ReadOp, ReadOpInner};
use crate::system::LocalPath;
use crate::{Error, FilePath, Operation, Reason};

impl FilePath {
    //! Read

    /// Opens a read operation to the file.
    pub fn read(&self) -> Result<ReadOp, Error> {
        if let Some(read) = self.read_if_exists()? {
            Ok(read)
        } else {
            Err(Error::new(
                self.clone(),
                Operation::Read,
                Reason::FileNotFound,
            ))
        }
    }

    /// Opens a read operation to the file if it exists.
    ///
    /// Returns `Ok(None)` if the file does not exist.
    pub fn read_if_exists(&self) -> Result<Option<ReadOp>, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.read_if_exists().map(|op| {
                op.map(|op| ReadOp {
                    inner: ReadOpInner::Local(op),
                })
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::from(self.path()) {
            return path.read_if_exists().map(|op| {
                op.map(|op| ReadOp {
                    inner: ReadOpInner::R2(op),
                })
            });
        }

        Err(Error::new(
            self.clone(),
            Operation::Read,
            Reason::UnknownFileSystem,
        ))
    }
}
