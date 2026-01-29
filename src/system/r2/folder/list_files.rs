use crate::system::r2::folder::R2ListFilesOp;
use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Error;

impl<'a> R2Path<'a> {
    //! List Files

    /// Lists the files in the folder.
    pub fn list_files(&self) -> Result<R2ListFilesOp, Error> {
        RUNTIME.block_on(async { self.list_files_async().await })
    }

    /// Lists the files in the folder.
    pub async fn list_files_async(&self) -> Result<R2ListFilesOp, Error> {
        R2ListFilesOp::from_async(self.path.clone().to_folder()?, *self).await
    }
}
