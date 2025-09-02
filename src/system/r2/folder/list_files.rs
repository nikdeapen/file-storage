use crate::system::R2Path;
use crate::{Error, FilePath};

impl<'a> R2Path<'a> {
    //! List Files

    /// See `FolderPath::list_files_to_vec`.
    pub fn list_files_to_vec(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.list_files_to_vec_async(target))
    }

    /// See `FolderPath::list_files_to_vec`.
    pub async fn list_files_to_vec_async(
        &self,
        target: &mut Vec<FilePath>,
    ) -> Result<usize, Error> {
        use crate::error::Operation::ListFiles;
        use crate::StoragePath;

        let response = Self::get_client(self.account_id)
            .await
            .list_objects()
            .bucket(self.bucket)
            .prefix(self.key)
            .send()
            .await;
        match response {
            Ok(response) => {
                let original_len: usize = target.len();
                if let Some(contents) = response.contents {
                    for object in contents {
                        if let Some(key) = object.key {
                            let s: String = format!(
                                "{}{}{}{}/{}",
                                Self::HTTPS_PREFIX,
                                self.account_id,
                                Self::R2_PREFIX,
                                self.bucket,
                                key
                            );
                            let path: StoragePath = StoragePath::parse(s)
                                .map_err(|e| Error::from_source(self.path.clone(), ListFiles, e))?;
                            if path.is_file() {
                                target.push(path.to_file()?)
                            }
                        }
                    }
                }
                Ok(target.len() - original_len)
            }
            Err(error) => Err(Error::from_source(
                self.path.clone(),
                ListFiles,
                std::io::Error::other(error),
            )),
        }
    }
}
