use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::primitives::ByteStream;
use bytes::{Buf, Bytes};
use std::io;
use std::io::Read;

pub struct R2FileRead {
    pub(crate) content: ByteStream,
    pub(crate) current: Bytes,
}

impl From<GetObjectOutput> for R2FileRead {
    fn from(object: GetObjectOutput) -> Self {
        Self {
            content: object.body,
            current: Bytes::default(),
        }
    }
}

impl R2FileRead {
    //! Read

    pub(crate) async fn read_async(&mut self, target: &mut [u8]) -> Result<usize, io::Error> {
        if !self.current.is_empty() {
            Ok(self.read_from_current(target))
        } else if let Some(content) = self.content.next().await {
            match content {
                Ok(content) => {
                    self.current = content;
                    Ok(self.read_from_current(target))
                }
                Err(error) => Err(error.into()),
            }
        } else {
            Ok(0)
        }
    }

    fn read_from_current(&mut self, target: &mut [u8]) -> usize {
        let copied: usize = if self.current.len() <= target.len() {
            let copied: usize = self.current.len();
            self.current
                .copy_to_slice(&mut target[..self.current.len()]);
            copied
        } else {
            let copied: usize = target.len();
            self.current.copy_to_slice(&mut target[..]);
            copied
        };
        copied
    }
}

impl Read for R2FileRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.current.is_empty() {
            Ok(self.read_from_current(buf))
        } else {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?
                .block_on(self.read_async(buf))
        }
    }
}
