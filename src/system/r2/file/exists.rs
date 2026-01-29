use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Error;
use crate::Operation::Exists;

impl<'a> R2Path<'a> {
    //! Exists

    /// See `FilePath::exist`.
    pub fn exists(&self) -> Result<bool, Error> {
        RUNTIME.block_on(self.exists_async())
    }

    /// See `FilePath::exist`.
    pub async fn exists_async(&self) -> Result<bool, Error> {
        use aws_sdk_s3::error::SdkError;
        use std::io;
        use std::io::ErrorKind::Other;

        let response = Self::get_client(self.account_id)
            .await
            .head_object()
            .bucket(self.bucket)
            .key(self.key)
            .send()
            .await;
        match response {
            Ok(_response) => Ok(true),
            Err(error) => match error {
                SdkError::ServiceError(e) if e.err().is_not_found() => Ok(false),
                error => Err(Error::from_source(
                    self.path.clone(),
                    Exists,
                    io::Error::new(Other, error),
                )),
            },
        }
    }
}
