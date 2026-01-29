use crate::system::R2Path;
use crate::Error;
use crate::Operation::Write;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

impl<'a> R2Path<'a> {
    //! Write

    /// See `FilePath::write_data_if_not_exists`.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        let data: &[u8] = data.as_ref();
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.write_data_if_not_exists_async(data))
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
