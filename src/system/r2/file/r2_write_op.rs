use crate::system::r2::r2_path::RUNTIME;
use crate::system::r2::sdk_message;
use crate::system::R2Path;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::operation::upload_part::UploadPartOutput;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use bytes::Bytes;
use std::io;

/// An R2 write operation.
///
/// # Size
/// - Every part but the last holds `MIN_PART_SIZE` bytes. R2 needs them to be the same size.
/// - An object can hold at most `MAX_PARTS` parts. That is about 48.8 GiB.
/// - Writing past that returns an error and marks the operation as failed.
///
/// # Close
/// - The object is only created by `close`. It is not created if the file exists by then.
/// - A partial last part can only be sent by `close`. `flush` is not a durability point.
/// - Dropping the operation without calling `close` aborts the upload.
///
/// # Failure
/// - Once a part fails to upload, every later call returns an error.
/// - The object is never created with a part missing. Dropping the operation aborts the upload.
pub(crate) struct R2WriteOp {
    account_id: String,
    bucket: String,
    key: String,
    upload_id: String,

    /// The next part. It never holds more than `MIN_PART_SIZE` bytes.
    buffer: Vec<u8>,

    part_number: i32,
    completed_parts: Vec<CompletedPart>,
    completed: bool,
    failed: bool,
}

impl R2WriteOp {
    //! Constants

    /// The minimum part size for multipart uploads. (5 MiB)
    const MIN_PART_SIZE: usize = 5 * 1024 * 1024;

    /// The most parts a multipart upload can hold.
    const MAX_PARTS: i32 = 10_000;
}

impl R2WriteOp {
    //! Failure

    /// Checks that no part has failed to upload.
    fn check_not_failed(&self) -> io::Result<()> {
        if self.failed {
            Err(io::Error::other("an earlier part failed to upload"))
        } else {
            Ok(())
        }
    }
}

impl R2WriteOp {
    //! Upload

    /// Uploads the buffer if it holds a full part.
    ///
    /// # Note
    /// - A failed upload marks the operation as failed. The part's data is gone with it.
    fn upload_full_part(&mut self) -> io::Result<()> {
        if self.buffer.len() < Self::MIN_PART_SIZE {
            return Ok(());
        }
        let data: Vec<u8> = std::mem::take(&mut self.buffer);
        let result: io::Result<()> = RUNTIME.block_on(self.upload_part_async(data));
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    /// Uploads the next part.
    async fn upload_part_async(&mut self, data: Vec<u8>) -> Result<(), io::Error> {
        if self.part_number > Self::MAX_PARTS {
            return Err(io::Error::other(
                "the object is too large for a multipart upload",
            ));
        }
        let body: ByteStream = ByteStream::from(Bytes::from(data));
        let output: UploadPartOutput = R2Path::get_client(&self.account_id)
            .await
            .upload_part()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .part_number(self.part_number)
            .body(body)
            .send()
            .await
            .map_err(|e| io::Error::other(sdk_message(&e)))?;

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

impl R2WriteOp {
    //! Construction

    /// Creates the [R2WriteOp] by starting a multipart upload.
    pub(in crate::system::r2) async fn create(
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
            .map_err(|e| io::Error::other(sdk_message(&e)))?;

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
            failed: false,
        })
    }
}

impl R2WriteOp {
    //! Close

    /// Completes the multipart upload to R2, creating the object.
    ///
    /// # Note
    /// - This must be called. Dropping the operation instead aborts the upload.
    /// - It fails with `AlreadyExists` if the file was created after the operation was opened.
    pub(crate) fn close(mut self) -> Result<(), io::Error> {
        self.check_not_failed()?;
        let result: Result<(), io::Error> = RUNTIME.block_on(self.close_async());
        if result.is_ok() {
            self.completed = true;
        }
        result
    }

    /// Uploads the last part and completes the upload if the file does not exist.
    async fn close_async(&mut self) -> Result<(), io::Error> {
        // An empty file still needs one part.
        if !self.buffer.is_empty() || self.completed_parts.is_empty() {
            let data: Vec<u8> = std::mem::take(&mut self.buffer);
            self.upload_part_async(data).await?;
        }

        let parts: Vec<CompletedPart> = std::mem::take(&mut self.completed_parts);
        let upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();
        let result = R2Path::get_client(&self.account_id)
            .await
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .multipart_upload(upload)
            .if_none_match("*")
            .send()
            .await;
        match result {
            Ok(_output) => Ok(()),
            Err(error) if error.code() == Some("PreconditionFailed") => Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "the file already exists",
            )),
            Err(error) => Err(io::Error::other(sdk_message(&error))),
        }
    }
}

impl io::Write for R2WriteOp {
    /// Buffers as much of the `buf` as fits in the current part.
    ///
    /// # Note
    /// - A full part is uploaded before taking more data. An upload error takes none of the `buf`.
    /// - Each call takes at most `MIN_PART_SIZE` bytes. Use `write_all` to write everything.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.check_not_failed()?;
        if buf.is_empty() {
            return Ok(0);
        }
        self.upload_full_part()?;
        let len: usize = buf.len().min(Self::MIN_PART_SIZE - self.buffer.len());
        self.buffer.extend_from_slice(&buf[..len]);
        Ok(len)
    }

    /// Uploads the buffer if it holds a full part.
    ///
    /// # Note
    /// - This is not a durability point. A partial last part can only be sent by `close`.
    fn flush(&mut self) -> io::Result<()> {
        self.check_not_failed()?;
        self.upload_full_part()
    }
}

impl Drop for R2WriteOp {
    /// Aborts the multipart upload unless it was completed by `close`.
    ///
    /// # Note
    /// - The abort blocks until R2 answers.
    /// - Inside an async runtime or while panicking it runs in the background instead. It may not
    ///   finish before the process exits.
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        let account_id: String = std::mem::take(&mut self.account_id);
        let bucket: String = std::mem::take(&mut self.bucket);
        let key: String = std::mem::take(&mut self.key);
        let upload_id: String = std::mem::take(&mut self.upload_id);
        let abort = async move {
            let _result = R2Path::get_client(&account_id)
                .await
                .abort_multipart_upload()
                .bucket(&bucket)
                .key(&key)
                .upload_id(&upload_id)
                .send()
                .await;
        };
        let in_runtime: bool = tokio::runtime::Handle::try_current().is_ok();
        if in_runtime || std::thread::panicking() {
            RUNTIME.spawn(abort);
        } else {
            RUNTIME.block_on(abort);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::R2WriteOp;
    use std::io::Write;

    /// Creates an operation that never reaches R2.
    ///
    /// # Note
    /// - It is marked completed so dropping it does not abort an upload.
    fn offline_op() -> R2WriteOp {
        R2WriteOp {
            account_id: String::default(),
            bucket: String::default(),
            key: String::default(),
            upload_id: String::default(),
            buffer: Vec::default(),
            part_number: 1,
            completed_parts: Vec::default(),
            completed: true,
            failed: false,
        }
    }

    #[test]
    fn write_takes_at_most_one_part() {
        let mut op: R2WriteOp = offline_op();
        let data: Vec<u8> = vec![0u8; R2WriteOp::MIN_PART_SIZE + 10];

        assert_eq!(op.write(&data[..10]).unwrap(), 10);
        assert_eq!(op.write(&data).unwrap(), R2WriteOp::MIN_PART_SIZE - 10);
        assert_eq!(op.buffer.len(), R2WriteOp::MIN_PART_SIZE);
        assert_eq!(op.write(&[]).unwrap(), 0);
    }

    #[test]
    fn failed_op_rejects_everything() {
        let mut op: R2WriteOp = offline_op();
        op.failed = true;

        assert!(op.write(b"data").is_err());
        assert!(op.flush().is_err());
        assert!(op.buffer.is_empty());
    }

    #[test]
    fn too_many_parts_fails() {
        let mut op: R2WriteOp = offline_op();
        op.part_number = R2WriteOp::MAX_PARTS + 1;
        op.buffer = vec![0u8; R2WriteOp::MIN_PART_SIZE];

        assert!(op.write(b"data").is_err());
        assert!(op.failed);
    }
}
