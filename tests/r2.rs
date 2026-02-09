#[allow(dead_code)]
mod file_util;
#[allow(dead_code)]
mod folder_util;

#[cfg(feature = "r2")]
mod tests {
    use crate::{file_util, folder_util};
    use chrono::Utc;
    use file_storage::{FilePath, FolderPath, StoragePath};
    use std::error::Error;

    #[test]
    #[ignore]
    fn file() -> Result<(), Box<dyn Error>> {
        let account: &str = "4a4bba3e2df525df01c99bae8307cbc5";
        let bucket: &str = "file-storage";
        let folder: String = format!("https://{}.r2.cloudflarestorage.com/{}/", account, bucket);
        let folder: StoragePath = StoragePath::parse(folder)?;

        let folder: FolderPath = folder
            .with_appended(Utc::now().format("%Y%m%d-%H%M%S").to_string())
            .make_folder();
        let file: FilePath = folder.make_file("test.file")?;

        file_util::test_file(&file, false, false)?;

        Ok(())
    }

    #[test]
    #[ignore]
    fn folder() -> Result<(), Box<dyn Error>> {
        let account: &str = "4a4bba3e2df525df01c99bae8307cbc5";
        let bucket: &str = "file-storage";
        let folder: String = format!("https://{}.r2.cloudflarestorage.com/{}/", account, bucket);
        let folder: StoragePath = StoragePath::parse(folder)?;
        let folder: FolderPath = folder
            .with_appended(Utc::now().format("%Y%m%d-%H%M%S").to_string())
            .make_folder();

        folder_util::list_files(&folder)?;
        folder_util::delete_files(&folder)?;

        Ok(())
    }
}
