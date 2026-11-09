use crate::Operation;
#[cfg(feature = "r2")]
use crate::R2WriteOp;
use crate::Reason::FileAlreadyExists;
use crate::{Error, FilePath, LocalWriteOp};
use std::io;

/// A write operation.
///
/// # Close
/// - The file only appears when `close` succeeds. `flush` is not a durability point.
/// - Dropping the operation without calling `close` discards the write.
///
/// # Local File Systems
/// - The data goes to a hidden temp file in the same folder until `close`. Listing the folder
///   shows it.
/// - A crash can leave the temp file behind. It is named `.file-storage-{pid}-{n}.tmp`.
/// - Creating a file needs hard links. File systems without them, such as FAT, fail at `close`.
/// - On Unix `close` syncs the file's folders after moving it into place. If that fails, the file
///   is in place but `close` still returns the error.
///
/// # Cloudflare R2
/// - `write` may take only part of the buffer. Use `write_all` to write everything.
/// - An object can hold at most about 48.8 GiB.
pub struct WriteOp {
    /// The file being written.
    pub(in crate::op) file: FilePath,

    pub(in crate::op) inner: WriteOpInner,
}

/// A file-system-specific write operation.
pub(in crate::op) enum WriteOpInner {
    /// A local write operation.
    Local(LocalWriteOp),

    /// A Cloudflare R2 write operation.
    #[cfg(feature = "r2")]
    R2(R2WriteOp),
}

impl io::Write for WriteOp {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.inner {
            WriteOpInner::Local(op) => op.write(buf),
            #[cfg(feature = "r2")]
            WriteOpInner::R2(op) => op.write(buf),
        }
    }

    /// Flushes what the backend is able to commit before the operation is closed.
    ///
    /// # Note
    /// - This is not a durability point. See the `Close` section on `WriteOp`.
    fn flush(&mut self) -> io::Result<()> {
        match &mut self.inner {
            WriteOpInner::Local(op) => op.flush(),
            #[cfg(feature = "r2")]
            WriteOpInner::R2(op) => op.flush(),
        }
    }
}

impl WriteOp {
    //! Close

    /// Ensures the file has been completely written and closes the file.
    ///
    /// # Note
    /// - This must be called. Dropping the operation instead discards the write.
    /// - A create-only write fails here with `FileAlreadyExists` if another writer created the
    ///   file first.
    pub fn close(self) -> Result<(), Error> {
        let result: io::Result<()> = match self.inner {
            WriteOpInner::Local(op) => op.close(),
            #[cfg(feature = "r2")]
            WriteOpInner::R2(op) => op.close(),
        };
        result.map_err(|error| {
            if error.kind() == io::ErrorKind::AlreadyExists {
                Error::new(self.file, Operation::Write, FileAlreadyExists)
            } else {
                Error::from_source(self.file, Operation::Write, error)
            }
        })
    }
}
