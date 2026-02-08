mod util;

#[cfg(feature = "tempfile")]
mod tests {
    use crate::util;
    use file_storage::{FilePath, FolderPath};
    use std::error::Error;

    #[test]
    fn file() -> Result<(), Box<dyn Error>> {
        let file: FilePath = FilePath::temp()?;

        util::exists(&file)?;

        util::delete(&file, true)?;

        util::read(&file)?;
        util::read_string(&file)?;
        util::read_vec(&file)?;

        util::write(&file)?;
        util::write_if_not_exists(&file)?;
        util::write_data(&file)?;

        Ok(())
    }

    #[test]
    fn folder() -> Result<(), Box<dyn Error>> {
        let file: FolderPath = FolderPath::temp()?;

        util::list_files(&file)?;
        util::delete_files(&file)?;

        Ok(())
    }
}
