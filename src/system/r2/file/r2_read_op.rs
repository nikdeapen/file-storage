use crate::system::r2::r2_path::RUNTIME;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::primitives::ByteStream;
use bytes::{Buf, Bytes};
use std::io;
use std::io::Read;

/// An R2 read operation.
pub(crate) struct R2ReadOp {
    content: ByteStream,
    current: Bytes,
}

impl From<GetObjectOutput> for R2ReadOp {
    fn from(object: GetObjectOutput) -> Self {
        Self {
            content: object.body,
            current: Bytes::default(),
        }
    }
}

impl R2ReadOp {
    //! Read

    /// Reads from the object into the `target`.
    ///
    /// # Note
    /// - Empty chunks are skipped. Only the end of the stream returns `Ok(0)` for a non-empty
    ///   `target`.
    async fn read_async(&mut self, target: &mut [u8]) -> Result<usize, io::Error> {
        while self.current.is_empty() {
            match self.content.next().await {
                Some(Ok(content)) => self.current = content,
                Some(Err(error)) => return Err(error.into()),
                None => return Ok(0),
            }
        }
        Ok(self.read_from_current(target))
    }

    /// Reads from the current chunk into the `target`.
    fn read_from_current(&mut self, target: &mut [u8]) -> usize {
        let len: usize = self.current.len().min(target.len());
        self.current.copy_to_slice(&mut target[..len]);
        len
    }
}

impl Read for R2ReadOp {
    fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
        if !self.current.is_empty() {
            Ok(self.read_from_current(target))
        } else {
            RUNTIME.block_on(self.read_async(target))
        }
    }
}
