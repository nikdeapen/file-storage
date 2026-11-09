use crate::system::r2::folder::R2ListFilesOp;
use crate::system::r2::r2_path::RUNTIME;
use crate::system::R2Path;
use crate::Error;

impl<'a> R2Path<'a> {
    //! List Files

    /// See `FolderPath::list_files`.
    pub(crate) fn list_files(self) -> Result<R2ListFilesOp, Error> {
        RUNTIME.block_on(R2ListFilesOp::from_async(self))
    }
}
