use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Operation::Exists;
use crate::{Error, Operation};
use aws_sdk_s3::error::SdkError;

impl<'a> R2Path<'a> {
    //! Exists

    /// See `FilePath::exists`.
    pub(crate) fn exists(self) -> Result<bool, Error> {
        RUNTIME.block_on(self.has_file(Exists))
    }

    /// Checks if the file exists, as part of the `operation`.
    ///
    /// # Note
    /// - R2 answers HEAD for a missing bucket with the same 404 as a missing key. Both are `false`.
    pub(in crate::system::r2) async fn has_file(self, operation: Operation) -> Result<bool, Error> {
        let response = Self::get_client(self.account_id)
            .await
            .head_object()
            .bucket(self.bucket)
            .key(self.key)
            .send()
            .await;
        match response {
            Ok(_response) => Ok(true),
            Err(SdkError::ServiceError(e)) if e.err().is_not_found() => Ok(false),
            Err(error) => Err(self.sdk_error(operation, error)),
        }
    }
}
