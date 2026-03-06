use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use bytes::Bytes;
use std::io;

/// The minimum part size for multipart uploads (5 MiB).
const MIN_PART_SIZE: usize = 5 * 1024 * 1024;

/// An R2 write operation.
///
/// Streams data to R2 via multipart upload.
pub struct R2WriteOp {
    account_id: String,
    bucket: String,
    key: String,
    upload_id: String,
    buffer: Vec<u8>,
    part_number: i32,
    completed_parts: Vec<CompletedPart>,
    completed: bool,
}

impl R2WriteOp {
    //! Create

    /// Creates a new multipart upload.
    pub(crate) async fn create(
        account_id: &str,
        bucket: &str,
        key: &str,
    ) -> Result<Self, io::Error> {
        let response = R2Path::get_client(account_id)
            .await
            .create_multipart_upload()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(io::Error::other)?;

        let upload_id: String = response
            .upload_id()
            .ok_or_else(|| io::Error::other("missing upload id"))?
            .to_string();

        Ok(Self {
            account_id: account_id.to_string(),
            bucket: bucket.to_string(),
            key: key.to_string(),
            upload_id,
            buffer: Vec::new(),
            part_number: 1,
            completed_parts: Vec::new(),
            completed: false,
        })
    }
}

impl R2WriteOp {
    //! Upload Part

    async fn upload_part_async(&mut self, data: Vec<u8>) -> Result<(), io::Error> {
        let body: ByteStream = ByteStream::from(Bytes::from(data));
        let output = R2Path::get_client(&self.account_id)
            .await
            .upload_part()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .part_number(self.part_number)
            .body(body)
            .send()
            .await
            .map_err(io::Error::other)?;

        self.completed_parts.push(
            CompletedPart::builder()
                .part_number(self.part_number)
                .set_e_tag(output.e_tag().map(|s| s.to_string()))
                .build(),
        );
        self.part_number += 1;
        Ok(())
    }
}

impl io::Write for R2WriteOp {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        while self.buffer.len() >= MIN_PART_SIZE {
            let remainder: Vec<u8> = self.buffer.split_off(MIN_PART_SIZE);
            let part_data: Vec<u8> = std::mem::replace(&mut self.buffer, remainder);
            RUNTIME.block_on(self.upload_part_async(part_data))?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl R2WriteOp {
    //! Close

    /// Completes the multipart upload to R2.
    pub fn close(mut self) -> Result<(), io::Error> {
        let result: Result<(), io::Error> = RUNTIME.block_on(self.close_async());
        if result.is_ok() {
            self.completed = true;
        }
        result
    }

    async fn close_async(&mut self) -> Result<(), io::Error> {
        // Upload remaining buffer as the final part.
        if !self.buffer.is_empty() || self.completed_parts.is_empty() {
            let data: Vec<u8> = std::mem::take(&mut self.buffer);
            self.upload_part_async(data).await?;
        }

        // Complete the multipart upload.
        let parts: Vec<CompletedPart> = std::mem::take(&mut self.completed_parts);
        let upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        R2Path::get_client(&self.account_id)
            .await
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .multipart_upload(upload)
            .send()
            .await
            .map_err(io::Error::other)?;

        Ok(())
    }
}

impl Drop for R2WriteOp {
    fn drop(&mut self) {
        if !self.completed {
            let account_id: String = std::mem::take(&mut self.account_id);
            let bucket: String = std::mem::take(&mut self.bucket);
            let key: String = std::mem::take(&mut self.key);
            let upload_id: String = std::mem::take(&mut self.upload_id);
            RUNTIME.spawn(async move {
                let _result = R2Path::get_client(&account_id)
                    .await
                    .abort_multipart_upload()
                    .bucket(&bucket)
                    .key(&key)
                    .upload_id(&upload_id)
                    .send()
                    .await;
            });
        }
    }
}
