use crate::system::r2::r2_path::RUNTIME;
use crate::system::r2::sdk_message;
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
pub(crate) struct R2ListFilesOp {
    root: FolderPath,

    /// The bucket's root folder. Each key is appended to it.
    bucket_root: StoragePath,

    /// The files left from the current page, in reverse order.
    files: Vec<FilePath>,
    finished: bool,
    paginator:
        PaginationStream<Result<ListObjectsV2Output, SdkError<ListObjectsV2Error, HttpResponse>>>,
}

impl R2ListFilesOp {
    //! Construction

    /// Creates the [R2ListFilesOp].
    pub(in crate::system::r2) async fn from_async(path: R2Path<'_>) -> Result<Self, Error> {
        let root: FolderPath = path.path.clone().to_folder()?;
        let bucket_root: StoragePath = StoragePath::parse(root.base_path())
            .map_err(|e| Error::from_source(root.clone(), ListFiles, e))?;
        let client: Client = R2Path::get_client(path.account_id).await;
        let paginator: PaginationStream<
            Result<ListObjectsV2Output, SdkError<ListObjectsV2Error, HttpResponse>>,
        > = client
            .list_objects_v2()
            .bucket(path.bucket)
            .prefix(path.key)
            .into_paginator()
            .send();
        Ok(Self {
            root,
            bucket_root,
            files: Vec::default(),
            finished: false,
            paginator,
        })
    }
}

impl R2ListFilesOp {
    //! Pagination

    /// Fetches the next page of results from R2.
    ///
    /// # Note
    /// - It is `true` if a page was fetched. It is `false` if there are no more pages.
    /// - Keys ending with `/` are folder markers, not files. They are skipped.
    fn fetch_next_page(&mut self) -> Result<bool, Error> {
        RUNTIME.block_on(async {
            let page = self.paginator.try_next().await.map_err(|error| {
                Error::from_message(self.root.clone(), ListFiles, sdk_message(&error))
            })?;
            let Some(page) = page else {
                return Ok(false);
            };
            let mut files: Vec<FilePath> = page
                .contents
                .unwrap_or_default()
                .into_iter()
                .filter(|object| !object.key().is_some_and(|key| key.ends_with('/')))
                .map(|object| self.to_file(object))
                .collect::<Result<Vec<FilePath>, Error>>()?;
            files.reverse();
            self.files = files;
            Ok(true)
        })
    }

    /// Converts the `object` to a file path.
    fn to_file(&self, object: Object) -> Result<FilePath, Error> {
        let key: String = object.key.ok_or_else(|| {
            Error::from_message(self.root.clone(), ListFiles, "missing object key")
        })?;
        self.bucket_root
            .clone_append(&key)
            .to_file()
            .map_err(|error| {
                Error::from_message(
                    self.root.clone(),
                    ListFiles,
                    format!("not a file path: {}", error.path()),
                )
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
            if self.finished {
                return None;
            }
            match self.fetch_next_page() {
                Ok(true) => continue,
                Ok(false) => {
                    self.finished = true;
                    return None;
                }
                Err(error) => {
                    self.finished = true;
                    self.files.clear();
                    return Some(Err(error));
                }
            }
        }
    }
}
