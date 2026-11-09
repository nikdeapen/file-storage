use crate::system::r2::r2_path::RUNTIME;
use crate::system::{R2Path, R2ReadOp};
use crate::Error;
use crate::Operation::Read;
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::primitives::ByteStream;

impl<'a> R2Path<'a> {
    //! Read

    /// See `FilePath::read_if_exists`.
    pub(crate) fn read_if_exists(self) -> Result<Option<R2ReadOp>, Error> {
        RUNTIME.block_on(self.read_if_exists_async())
    }

    /// See `FilePath::read_to_vec_if_exists`.
    pub(crate) fn read_to_vec_if_exists(
        self,
        target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        RUNTIME.block_on(self.read_to_vec_if_exists_async(target))
    }

    /// See `FilePath::read_if_exists`.
    async fn read_if_exists_async(self) -> Result<Option<R2ReadOp>, Error> {
        Ok(self.get_object().await?.map(R2ReadOp::from))
    }

    /// See `FilePath::read_to_vec_if_exists`.
    ///
    /// # Note
    /// - The object's size is reserved up front. An object too large to hold is an error.
    async fn read_to_vec_if_exists_async(
        self,
        target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        let Some(object) = self.get_object().await? else {
            return Ok(None);
        };
        let mut body: ByteStream = object.body;
        let size: usize = usize::try_from(body.size_hint().0).unwrap_or(usize::MAX);
        target
            .try_reserve(size)
            .map_err(|error| Error::from_source(self.path.clone(), Read, error))?;
        let mut total: usize = 0;
        while let Some(chunk) = body
            .try_next()
            .await
            .map_err(|error| Error::from_source(self.path.clone(), Read, error))?
        {
            target.extend_from_slice(&chunk);
            total += chunk.len();
        }
        Ok(Some(total))
    }

    /// Gets the object.
    ///
    /// # Note
    /// - It is `None` if the object does not exist.
    /// - A missing bucket is an error. A GET tells it apart from a missing key. A HEAD cannot.
    async fn get_object(self) -> Result<Option<GetObjectOutput>, Error> {
        let result: Result<GetObjectOutput, SdkError<GetObjectError>> =
            Self::get_client(self.account_id)
                .await
                .get_object()
                .bucket(self.bucket)
                .key(self.key)
                .send()
                .await;
        match result {
            Ok(object) => Ok(Some(object)),
            Err(error)
                if matches!(error.as_service_error(), Some(GetObjectError::NoSuchKey(_))) =>
            {
                Ok(None)
            }
            Err(error) if error.code() == Some("NoSuchBucket") => {
                let message: String = format!("the bucket does not exist: {}", self.bucket);
                Err(Error::from_message(self.path.clone(), Read, message))
            }
            Err(error) => Err(self.sdk_error(Read, error)),
        }
    }
}
