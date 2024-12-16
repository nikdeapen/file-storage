use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

use crate::path::r2::R2Path;
use crate::Error;
use crate::Operation::Read;

impl<'a> R2Path<'a> {
    //! Read

    /// See `FilePath::read_to_vec_if_exists`.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.read_to_vec_if_exists_async(target))
    }

    /// See `R2Path::read_to_vec_if_exists`.
    pub async fn read_to_vec_if_exists_async(
        &self,
        target: &mut Vec<u8>,
    ) -> Result<Option<usize>, Error> {
        let response = Self::get_client(self.account_id)
            .await
            .get_object()
            .bucket(self.bucket)
            .key(self.key)
            .send()
            .await;
        match response {
            Ok(response) => {
                let size_hint: u64 = response.body.size_hint().0;
                if size_hint <= usize::MAX as u64 {
                    target.reserve(size_hint as usize);
                }
                let mut body: ByteStream = response.body;
                let original_len: usize = target.len();
                while let Some(chunk) = body.next().await {
                    let chunk: Bytes =
                        chunk.map_err(|error| Error::from_cause(self.path.clone(), Read, error))?;
                    target.extend_from_slice(&chunk);
                }
                Ok(Some(target.len() - original_len))
            }
            Err(error) => {
                if matches!(error.as_service_error(), Some(GetObjectError::NoSuchKey(_))) {
                    Ok(None)
                } else {
                    Err(Error::from_cause(
                        self.path.clone(),
                        Read,
                        std::io::Error::new(std::io::ErrorKind::Other, error),
                    ))
                }
            }
        }
    }
}
