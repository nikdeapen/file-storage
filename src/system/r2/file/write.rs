use crate::system::r2::r2_path::RUNTIME;
use crate::system::{R2Path, R2WriteOp};
use crate::Operation::Write;
use crate::{Error, Headers};
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_s3::operation::put_object::builders::PutObjectFluentBuilder;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

impl<'a> R2Path<'a> {
    //! Write

    /// See `FilePath::write_if_not_exists`.
    pub(crate) fn write_if_not_exists(self) -> Result<Option<R2WriteOp>, Error> {
        RUNTIME.block_on(self.write_if_not_exists_async())
    }

    /// See `FilePath::write_data_if_not_exists`.
    pub(crate) fn write_data_if_not_exists<D>(self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        self.write_with_headers_if_not_exists(data, &Headers::new())
    }

    /// See `FilePath::write_if_not_exists`.
    ///
    /// # Note
    /// - The check here only finds an existing file early. The atomic step is in `close`, which
    ///   completes the upload only if the file still does not exist.
    async fn write_if_not_exists_async(self) -> Result<Option<R2WriteOp>, Error> {
        if self.has_file(Write).await? {
            return Ok(None);
        }
        let op: R2WriteOp = R2WriteOp::create(self.account_id, self.bucket, self.key)
            .await
            .map_err(|e| Error::from_source(self.path.clone(), Write, e))?;
        Ok(Some(op))
    }
}

impl<'a> R2Path<'a> {
    //! Write With Headers

    /// See `FilePath::overwrite_with_headers`.
    pub(crate) fn overwrite_with_headers<D>(self, data: D, headers: &Headers) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        RUNTIME.block_on(self.overwrite_with_headers_async(data.as_ref(), headers))
    }

    /// See `FilePath::write_with_headers_if_not_exists`.
    pub(crate) fn write_with_headers_if_not_exists<D>(
        self,
        data: D,
        headers: &Headers,
    ) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        RUNTIME.block_on(self.write_with_headers_if_not_exists_async(data.as_ref(), headers))
    }

    /// See `FilePath::overwrite_with_headers`.
    async fn overwrite_with_headers_async(
        self,
        data: &[u8],
        headers: &Headers,
    ) -> Result<(), Error> {
        match self.put_request(data, headers).await.send().await {
            Ok(_output) => Ok(()),
            Err(error) => Err(self.sdk_error(Write, error)),
        }
    }

    /// See `FilePath::write_with_headers_if_not_exists`.
    async fn write_with_headers_if_not_exists_async(
        self,
        data: &[u8],
        headers: &Headers,
    ) -> Result<bool, Error> {
        let request: PutObjectFluentBuilder =
            self.put_request(data, headers).await.if_none_match("*");
        match request.send().await {
            Ok(_output) => Ok(true),
            Err(SdkError::ServiceError(error))
                if error.err().code() == Some("PreconditionFailed") =>
            {
                Ok(false)
            }
            Err(error) => Err(self.sdk_error(Write, error)),
        }
    }

    /// Builds a `put_object` request with the `data` body and the `headers`.
    ///
    /// # Note
    /// - Known headers map to their typed setters. Any others become object metadata.
    async fn put_request(self, data: &[u8], headers: &Headers) -> PutObjectFluentBuilder {
        let body: ByteStream = ByteStream::from(Bytes::copy_from_slice(data));
        let mut request: PutObjectFluentBuilder = Self::get_client(self.account_id)
            .await
            .put_object()
            .bucket(self.bucket)
            .key(self.key)
            .body(body);
        for (name, value) in headers.iter() {
            let is = |known: &str| name.eq_ignore_ascii_case(known);
            request = if is("content-type") {
                request.content_type(value)
            } else if is("cache-control") {
                request.cache_control(value)
            } else if is("content-encoding") {
                request.content_encoding(value)
            } else if is("content-disposition") {
                request.content_disposition(value)
            } else if is("content-language") {
                request.content_language(value)
            } else {
                request.metadata(name, value)
            };
        }
        request
    }
}
