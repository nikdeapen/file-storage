use crate::system::r2::r2_path::RUNTIME;
use crate::system::{R2Path, R2WriteOp};
use crate::Error;
use crate::Headers;
use crate::Operation::Write;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::builders::PutObjectFluentBuilder;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

impl<'a> R2Path<'a> {
    //! Write

    /// See `FilePath::write_if_not_exists`.
    pub fn write_if_not_exists(self) -> Result<Option<R2WriteOp>, Error> {
        RUNTIME.block_on(self.write_if_not_exists_async())
    }

    /// See `FilePath::write_if_not_exists`.
    async fn write_if_not_exists_async(self) -> Result<Option<R2WriteOp>, Error> {
        if self.exists_async().await? {
            return Ok(None);
        }
        let op: R2WriteOp = R2WriteOp::create(self.account_id, self.bucket, self.key)
            .await
            .map_err(|e| Error::from_source(self.path.clone(), Write, e))?;
        Ok(Some(op))
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub fn write_data_if_not_exists<D>(self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        self.write_with_headers_if_not_exists(data, &Headers::default())
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub async fn write_data_if_not_exists_async<D>(self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        self.write_with_headers_if_not_exists_async(data.as_ref(), &Headers::default())
            .await
    }
}

impl<'a> R2Path<'a> {
    //! Write: Headers

    /// See `FilePath::write_with_headers`.
    pub fn write_with_headers<D>(self, data: D, headers: &Headers) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        RUNTIME.block_on(self.write_with_headers_async(data.as_ref(), headers))
    }

    /// See `FilePath::write_with_headers`.
    pub async fn write_with_headers_async(self, data: &[u8], headers: &Headers) -> Result<(), Error> {
        let request: PutObjectFluentBuilder = self.put_request(data, headers).await;
        match request.send().await {
            Ok(_response) => Ok(()),
            Err(error) => Err(Error::from_source(
                self.path.clone(),
                Write,
                std::io::Error::other(error),
            )),
        }
    }

    /// See `FilePath::write_with_headers_if_not_exists`.
    pub fn write_with_headers_if_not_exists<D>(
        self,
        data: D,
        headers: &Headers,
    ) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        RUNTIME.block_on(self.write_with_headers_if_not_exists_async(data.as_ref(), headers))
    }

    /// See `FilePath::write_with_headers_if_not_exists`.
    pub async fn write_with_headers_if_not_exists_async(
        self,
        data: &[u8],
        headers: &Headers,
    ) -> Result<bool, Error> {
        let request: PutObjectFluentBuilder = self
            .put_request(data, headers)
            .await
            .set_if_none_match(Some("*".to_string()));
        match request.send().await {
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

    /// Builds a `put_object` request with the `data` body and `headers` applied.
    ///
    /// Recognized response headers map to their typed setters; any others are set as object
    /// metadata.
    async fn put_request(self, data: &[u8], headers: &Headers) -> PutObjectFluentBuilder {
        let body: ByteStream = ByteStream::from(Bytes::copy_from_slice(data));
        let mut request: PutObjectFluentBuilder = Self::get_client(self.account_id)
            .await
            .put_object()
            .bucket(self.bucket)
            .key(self.key)
            .body(body);
        for (name, value) in headers.iter() {
            request = match name.to_ascii_lowercase().as_str() {
                "content-type" => request.content_type(value),
                "cache-control" => request.cache_control(value),
                "content-encoding" => request.content_encoding(value),
                "content-disposition" => request.content_disposition(value),
                "content-language" => request.content_language(value),
                _ => request.metadata(name, value),
            };
        }
        request
    }
}
