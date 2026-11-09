use crate::op::file::read::read_op::{ReadOp, ReadOpInner};
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::{Error, FilePath, LocalPath, Operation, Reason};

impl FilePath {
    //! Read

    /// Opens a read operation to the file.
    ///
    /// # Note
    /// - A missing file fails with `FileNotFound`. See `read_if_exists` for what counts as missing.
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
    /// # Note
    /// - It is `None` if the file does not exist.
    /// - A local folder at the path is not a file. It is `None`, like in `exists`.
    /// - Anything else at a local path, such as a FIFO or a device, is an error.
    /// - On Cloudflare R2 a missing bucket is an error. `exists` returns `false` for it.
    pub fn read_if_exists(&self) -> Result<Option<ReadOp>, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.read_if_exists().map(|op| {
                op.map(|op| ReadOp {
                    inner: ReadOpInner::Local(op),
                })
            });
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
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
