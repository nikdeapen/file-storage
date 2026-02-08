mod file_util;
mod folder_util;

#[cfg(feature = "tempfile")]
mod tests {
    use crate::file_util;
    use crate::folder_util;
    use file_storage::{FilePath, FolderPath};
    use std::error::Error;

    #[test]
    fn file() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;
        file_util::test_file(&file, true, true)?;
        Ok(())
    }

    #[test]
    fn folder() -> Result<(), Box<dyn Error>> {
        let file: FolderPath = FolderPath::temp()?;

        folder_util::list_files(&file)?;
        folder_util::delete_files(&file)?;

        Ok(())
    }
}
