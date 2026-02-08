use crate::system::r2::r2_path::RUNTIME;
use crate::system::{R2Path, R2ReadOp};
use crate::Error;
use crate::Operation::Read;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};

impl<'a> R2Path<'a> {
    //! Read

    async fn get_object_output(&self) -> Result<Option<GetObjectOutput>, Error> {
        let result: Result<GetObjectOutput, SdkError<GetObjectError>> =
            Self::get_client(self.account_id)
                .await
                .get_object()
                .bucket(self.bucket)
                .key(self.key)
                .send()
                .await;
        match result {
            Ok(get_object_output) => Ok(Some(get_object_output)),
            Err(error) => {
                if matches!(error.as_service_error(), Some(GetObjectError::NoSuchKey(_))) {
                    Ok(None)
                } else {
                    Err(Error::from_source(
                        self.path.clone(),
                        Read,
                        std::io::Error::other(error),
                    ))
                }
            }
        }
    }

    async fn read_if_exists_async(&self) -> Result<Option<R2ReadOp>, Error> {
        if let Some(object) = self.get_object_output().await? {
            Ok(Some(R2ReadOp::from(object)))
        } else {
            Ok(None)
        }
    }

    pub fn read_if_exists(&self) -> Result<Option<R2ReadOp>, Error> {
        RUNTIME.block_on(self.read_if_exists_async())
    }

    /// See `FilePath::read_to_vec_if_exists`.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        RUNTIME.block_on(self.read_to_vec_if_exists_async(target))
    }

    /// See `FilePath::read_to_vec_if_exists`.
    pub async fn read_to_vec_if_exists_async(
        &self,
        target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        if let Some(object) = self.get_object_output().await? {
            let mut read: R2ReadOp = R2ReadOp::from(object);
            let min_size: u64 = read.content.size_hint().0;
            if min_size > 0 && min_size <= usize::MAX as u64 {
                target.reserve(min_size as usize);
            }
            let mut total: usize = 0;
            loop {
                let mut buffer: [u8; 4096] = [0; 4096];
                let read: usize = read
                    .read_async(&mut buffer)
                    .await
                    .map_err(|error| Error::from_source(self.path.clone(), Read, error))?;
                if read == 0 {
                    return Ok(Some(total));
                } else {
                    target.extend_from_slice(&buffer[..read]);
                }
                total += read;
            }
        } else {
            Ok(None)
        }
    }
}
