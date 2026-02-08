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
    files: Vec<FilePath>,
    paginator:
        PaginationStream<Result<ListObjectsV2Output, SdkError<ListObjectsV2Error, HttpResponse>>>,
}

impl R2ListFilesOp {
    //! Construction

    /// Creates a new list-file operation.
    pub async fn from_async(root: FolderPath, path: R2Path<'_>) -> Result<Self, Error> {
        let client: Client = R2Path::get_client(path.account_id).await;
        let paginator = client
            .list_objects_v2()
            .bucket(path.bucket)
            .prefix(path.key)
            .into_paginator()
            .send();
        Ok(R2ListFilesOp {
            root,
            files: Vec::default(),
            paginator,
        })
    }
}

impl R2ListFilesOp {
    //! Conversions

    /// Converts the `object` to a file path.
    fn to_file(&self, object: Object) -> Result<FilePath, Error> {
        if let Some(key) = object.key {
            let path: R2Path = R2Path::from(self.root.path()).ok_or(Error::from_message(
                self.root.clone(),
                ListFiles,
                "invalid R2 path",
            ))?;
            let path: String = path.path_with_object_key(key.as_str());
            let path: StoragePath = StoragePath::parse(path)
                .map_err(|e| Error::from_source(self.root.clone(), ListFiles, e))?;
            match path.to_file() {
                Ok(file) => Ok(file),
                Err(error) => Err(Error::from_message(
                    self.root.clone(),
                    ListFiles,
                    format!("not a file path: {}", error.path()),
                )),
            }
        } else {
            Err(Error::from_message(
                self.root.clone(),
                ListFiles,
                "missing object key",
            ))
        }
    }
}

impl Iterator for R2ListFilesOp {
    type Item = Result<FilePath, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return if let Some(next) = self.files.pop() {
                Some(Ok(next))
            } else {
                let result: Option<Result<Vec<FilePath>, Error>> = RUNTIME.block_on(async {
                    match self.paginator.try_next().await {
                        Ok(next) => {
                            if let Some(next) = next {
                                let files: Result<Vec<FilePath>, Error> = next
                                    .contents
                                    .unwrap_or_default()
                                    .drain(..)
                                    .map(|o| self.to_file(o))
                                    .collect::<Result<Vec<FilePath>, Error>>();
                                match files {
                                    Ok(files) => Some(Ok(files)),
                                    Err(error) => Some(Err(error)),
                                }
                            } else {
                                None
                            }
                        }
                        Err(error) => Some(Err(Error::from_source(
                            self.root.clone(),
                            ListFiles,
                            std::io::Error::other(error),
                        ))),
                    }
                });
                if let Some(result) = result {
                    match result {
                        Ok(files) => {
                            self.files = files;
                            self.files.reverse();
                            continue;
                        }
                        Err(error) => return Some(Err(error)),
                    }
                } else {
                    return None;
                }
            };
        }
    }
}
