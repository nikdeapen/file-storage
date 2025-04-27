mod common;

#[cfg(feature = "tempfile")]
mod tests {
    use crate::common::delete::{test_delete, test_delete_if_exist};
    use crate::common::exists::test_exists;

    use crate::common::delete_files::test_delete_files;
    use crate::common::list_files::test_list_files;
    use crate::common::read::test_read;
    use crate::common::write::test_write;
    use file_storage::{FilePath, FolderPath};
    use std::error::Error;

    #[test]
    fn file() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;

        test_delete(&file)?;
        test_delete_if_exist(&file)?;
        test_exists(&file)?;
        test_read(&file)?;
        test_write(&file)?;

        Ok(())
    }

    #[test]
    fn folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::temp()?;

        test_list_files(&folder)?;
        test_delete_files(&folder)?;

        Ok(())
    }
}
