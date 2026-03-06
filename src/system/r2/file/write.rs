use crate::system::r2::r2_path::RUNTIME;
use crate::system::{R2Path, R2WriteOp};
use crate::Error;
use crate::Operation::Write;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

impl<'a> R2Path<'a> {
    //! Write

    /// See `FilePath::write_if_not_exists`.
    pub fn write_if_not_exists(&self) -> Result<Option<R2WriteOp>, Error> {
        RUNTIME.block_on(self.write_if_not_exists_async())
    }

    /// See `FilePath::write_if_not_exists`.
    async fn write_if_not_exists_async(&self) -> Result<Option<R2WriteOp>, Error> {
        if self.exists_async().await? {
            return Ok(None);
        }
        let op: R2WriteOp = R2WriteOp::create(self.account_id, self.bucket, self.key)
            .await
            .map_err(|e| Error::from_source(self.path.clone(), Write, e))?;
        Ok(Some(op))
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        let data: &[u8] = data.as_ref();
        RUNTIME.block_on(self.write_data_if_not_exists_async(data))
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub async fn write_data_if_not_exists_async<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        let body: ByteStream = ByteStream::from(Bytes::copy_from_slice(data.as_ref()));
        let response = Self::get_client(self.account_id)
            .await
            .put_object()
            .bucket(self.bucket)
            .key(self.key)
            .set_if_none_match(Some("*".to_string()))
            .body(body)
            .send()
            .await;
        match response {
            Ok(_response) => Ok(true),
            Err(SdkError::ServiceError(error))
                if error.err().code() == Some("PreconditionFailed") =>
            {
                Ok(false)
            }
            Err(error) => Err(Error::from_source(
                self.path.clone(),
                Write,
                std::io::Error::other(error),
            )),
        }
    }
}
