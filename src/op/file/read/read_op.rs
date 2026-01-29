use std::io::Read;

/// A read operation.
pub struct ReadOp {
    pub(crate) inner: ReadOpInner,
}

/// A file-system-specific read operation.
pub(crate) enum ReadOpInner {
    /// A local read operation.
    Local(crate::LocalReadOp),

    /// A cloudflare r2 read operation.
    #[cfg(feature = "r2")]
    R2(crate::R2ReadOp),
}

impl Read for ReadOp {
    //! Read

    /// See `std::io::Read::read`.
    fn read(&mut self, target: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReadOpInner::Local(op) => op.read(target),
            #[cfg(feature = "r2")]
            ReadOpInner::R2(op) => op.read(target),
        }
    }
}
