use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Operation::ListFiles;
use crate::{Error, FilePath, FolderPath, StoragePath};
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::list_objects_v2::{ListObjectsV2Error, ListObjectsV2Output};
use aws_sdk_s3::types::Object;
use aws_sdk_s3::Client;
use aws_smithy_async::future::pagination_stream::PaginationStream;

/// An R2 list-files operation.
pub struct R2ListFilesOp {
    root: FolderPath,
    base_url: String,
    files: Vec<FilePath>,
    paginator:
        PaginationStream<Result<ListObjectsV2Output, SdkError<ListObjectsV2Error, HttpResponse>>>,
}

impl R2ListFilesOp {
    //! Construction

    /// Creates a new list-files operation.
    pub async fn from_async(root: FolderPath, path: R2Path<'_>) -> Result<Self, Error> {
        let client: Client = R2Path::get_client(path.account_id).await;
        let paginator = client
            .list_objects_v2()
            .bucket(path.bucket)
            .prefix(path.key)
            .into_paginator()
            .send();
        let base_url: String = format!(
            "{}{}{}{}/",
            R2Path::HTTPS_PREFIX,
            path.account_id,
            R2Path::R2_PREFIX,
            path.bucket,
        );
        Ok(Self {
            root,
            base_url,
            files: Vec::default(),
            paginator,
        })
    }
}

impl R2ListFilesOp {
    //! Conversions

    /// Converts the `object` to a file path.
    fn to_file(&self, object: Object) -> Result<FilePath, Error> {
        let key: String = object.key.ok_or_else(|| {
            Error::from_message(self.root.clone(), ListFiles, "missing object key")
        })?;
        let path: String = format!("{}{}", self.base_url, key);
        let path: StoragePath = StoragePath::parse(path)
            .map_err(|e| Error::from_source(self.root.clone(), ListFiles, e))?;
        path.to_file().map_err(|error| {
            Error::from_message(
                self.root.clone(),
                ListFiles,
                format!("not a file path: {}", error.path()),
            )
        })
    }
}

impl R2ListFilesOp {
    //! Pagination

    /// Fetches the next page of results from R2.
    ///
    /// Returns `Ok(true)` if a page was fetched.
    /// Returns `Ok(false)` if there are no more pages.
    fn fetch_next_page(&mut self) -> Result<bool, Error> {
        RUNTIME.block_on(async {
            let page = self
                .paginator
                .try_next()
                .await
                .map_err(|error| {
                    Error::from_source(
                        self.root.clone(),
                        ListFiles,
                        std::io::Error::other(error),
                    )
                })?;
            let Some(page) = page else {
                return Ok(false);
            };
            let mut files: Vec<FilePath> = page
                .contents
                .unwrap_or_default()
                .into_iter()
                .map(|o| self.to_file(o))
                .collect::<Result<Vec<FilePath>, Error>>()?;
            files.reverse();
            self.files = files;
            Ok(true)
        })
    }
}

impl Iterator for R2ListFilesOp {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(file) = self.files.pop() {
                return Some(Ok(file));
            }
            match self.fetch_next_page() {
                Ok(true) => continue,
                Ok(false) => return None,
                Err(error) => return Some(Err(error)),
            }
        }
    }
}
