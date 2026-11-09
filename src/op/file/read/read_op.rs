use crate::LocalReadOp;
#[cfg(feature = "r2")]
use crate::R2ReadOp;
use std::io::Read;

/// A read operation.
pub struct ReadOp {
    pub(in crate::op) inner: ReadOpInner,
}

/// A file-system-specific read operation.
pub(in crate::op) enum ReadOpInner {
    /// A local read operation.
    Local(LocalReadOp),

    /// A Cloudflare R2 read operation.
    #[cfg(feature = "r2")]
    R2(R2ReadOp),
}

impl Read for ReadOp {
    fn read(&mut self, target: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            ReadOpInner::Local(op) => op.read(target),
            #[cfg(feature = "r2")]
            ReadOpInner::R2(op) => op.read(target),
        }
    }
}
