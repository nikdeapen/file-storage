mod file_util;

#[cfg(feature = "r2")]
mod tests {
    use crate::file_util;
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
}
