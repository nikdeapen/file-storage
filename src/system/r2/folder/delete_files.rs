use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Error;
use crate::Operation::DeleteFiles;
use aws_sdk_s3::error::BuildError;
use aws_sdk_s3::operation::delete_objects::DeleteObjectsOutput;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use aws_sdk_s3::Client;
use aws_smithy_async::future::pagination_stream::PaginationStream;

impl<'a> R2Path<'a> {
    //! Delete Files

    /// See `FolderPath::delete_files`.
    pub(crate) fn delete_files(self) -> Result<(), Error> {
        RUNTIME.block_on(self.delete_files_async())
    }

    /// See `FolderPath::delete_files`.
    async fn delete_files_async(self) -> Result<(), Error> {
        let build_error = |error: BuildError| {
            Error::from_source(self.path.clone(), DeleteFiles, std::io::Error::other(error))
        };
        let client: Client = Self::get_client(self.account_id).await;
        let mut pages: PaginationStream<_> = client
            .list_objects_v2()
            .bucket(self.bucket)
            .prefix(self.key)
            .into_paginator()
            .send();

        while let Some(page) = pages
            .try_next()
            .await
            .map_err(|e| self.sdk_error(DeleteFiles, e))?
        {
            let objects: Vec<ObjectIdentifier> = page
                .contents
                .unwrap_or_default()
                .into_iter()
                .filter_map(|object| object.key)
                .map(|key| ObjectIdentifier::builder().key(key).build())
                .collect::<Result<Vec<ObjectIdentifier>, _>>()
                .map_err(build_error)?;
            if objects.is_empty() {
                continue;
            }

            let delete: Delete = Delete::builder()
                .set_objects(Some(objects))
                .quiet(true)
                .build()
                .map_err(build_error)?;
            let output: DeleteObjectsOutput = client
                .delete_objects()
                .bucket(self.bucket)
                .delete(delete)
                .send()
                .await
                .map_err(|e| self.sdk_error(DeleteFiles, e))?;
            self.check_deleted(&output)?;
        }

        Ok(())
    }

    /// Checks that every object in a delete request was deleted.
    ///
    /// # Note
    /// - A delete request can succeed while some of its objects fail. The response lists them.
    fn check_deleted(self, output: &DeleteObjectsOutput) -> Result<(), Error> {
        let Some(first) = output.errors().first() else {
            return Ok(());
        };
        let message: String = format!(
            "{} of the files were not deleted. The first is {}: {}",
            output.errors().len(),
            first.key().unwrap_or_default(),
            first.message().or(first.code()).unwrap_or("unknown error"),
        );
        Err(Error::from_message(self.path.clone(), DeleteFiles, message))
    }
}
