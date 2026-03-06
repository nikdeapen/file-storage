use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Error;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};

impl<'a> R2Path<'a> {
    //! Delete Files

    /// Deletes all files with the current key prefix.
    pub fn delete_files(&self) -> Result<(), Error> {
        RUNTIME.block_on(self.delete_files_async())
    }

    /// Deletes all files with the current key prefix.
    pub async fn delete_files_async(&self) -> Result<(), Error> {
        let client = Self::get_client(self.account_id).await;
        let mut paginator = client
            .list_objects_v2()
            .bucket(self.bucket)
            .prefix(self.key)
            .into_paginator()
            .send();

        while let Some(page) = paginator
            .try_next()
            .await
            .map_err(|e| Error::from_source(self.path.clone(), crate::Operation::Delete, std::io::Error::other(e)))?
        {
            let objects: Vec<ObjectIdentifier> = page
                .contents
                .unwrap_or_default()
                .into_iter()
                .filter_map(|o| {
                    o.key.map(|k| {
                        ObjectIdentifier::builder()
                            .key(k)
                            .build()
                            .expect("key is set")
                    })
                })
                .collect();

            if objects.is_empty() {
                continue;
            }

            let delete: Delete = Delete::builder()
                .set_objects(Some(objects))
                .build()
                .map_err(|e| {
                    Error::from_source(self.path.clone(), crate::Operation::Delete, std::io::Error::other(e))
                })?;

            client
                .delete_objects()
                .bucket(self.bucket)
                .delete(delete)
                .send()
                .await
                .map_err(|e| {
                    Error::from_source(self.path.clone(), crate::Operation::Delete, std::io::Error::other(e))
                })?;
        }

        Ok(())
    }
}
