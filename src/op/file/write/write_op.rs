use crate::system::LocalWriteOp;
use std::io::Write;

/// A write operation.
pub struct WriteOp {
    pub(crate) inner: WriteOpInner,
}

/// A file-system-specific write operation.
pub(crate) enum WriteOpInner {
    /// A local write operation.
    Local(LocalWriteOp),

    /// A cloudflare R2 write operation.
    #[cfg(feature = "r2")]
    R2(crate::R2WriteOp),
}

impl Write for WriteOp {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            WriteOpInner::Local(op) => op.write(buf),
            #[cfg(feature = "r2")]
            WriteOpInner::R2(op) => op.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
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
    pub fn close(self) -> Result<(), std::io::Error> {
        match self.inner {
            WriteOpInner::Local(op) => op.close(),
            #[cfg(feature = "r2")]
            WriteOpInner::R2(op) => op.close(),
        }
    }
}
