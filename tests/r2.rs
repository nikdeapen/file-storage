#[allow(dead_code)]
mod file_util;
#[allow(dead_code)]
mod folder_util;

#[cfg(feature = "r2")]
mod tests {
    use crate::{file_util, folder_util};
    use chrono::Utc;
    use file_storage::{FilePath, FolderPath, StoragePath, WriteOp};
    use std::error::Error;
    use std::io::Write;

    /// Creates a new folder for one run of the test with the `name`.
    ///
    /// # Note
    /// - The account and bucket come from `FILE_STORAGE_R2_ACCOUNT` and `FILE_STORAGE_R2_BUCKET`.
    /// - The `name` keeps tests that start in the same second out of each other's folders.
    fn test_folder(name: &str) -> Result<FolderPath, Box<dyn Error>> {
        let account: String = std::env::var("FILE_STORAGE_R2_ACCOUNT")
            .map_err(|_| "set FILE_STORAGE_R2_ACCOUNT to run the R2 tests")?;
        let bucket: String = std::env::var("FILE_STORAGE_R2_BUCKET")
            .map_err(|_| "set FILE_STORAGE_R2_BUCKET to run the R2 tests")?;
        let folder: String = format!("https://{}.r2.cloudflarestorage.com/{}/", account, bucket);
        let run: String = format!("{}-{}", Utc::now().format("%Y%m%d-%H%M%S"), name);
        Ok(StoragePath::parse(folder)?.with_appended(run).make_folder())
    }

    #[test]
    #[ignore]
    fn file() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = test_folder("file")?;
        let file: FilePath = folder.clone_append("test.file").to_file()?;

        file_util::test_file(&file, false)?;

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn a_missing_bucket_is_an_error() -> Result<(), Box<dyn Error>> {
        let account: String = std::env::var("FILE_STORAGE_R2_ACCOUNT")
            .map_err(|_| "set FILE_STORAGE_R2_ACCOUNT to run the R2 tests")?;
        let path: String = format!(
            "https://{}.r2.cloudflarestorage.com/file-storage-no-such-bucket/test.file",
            account
        );
        let file: FilePath = StoragePath::parse(path)?.to_file()?;

        // A HEAD cannot tell a missing bucket from a missing file.
        assert!(!file.exists()?);
        let message: String = file.read_as_vec_if_exists().unwrap_err().to_string();
        assert!(message.contains("the bucket does not exist"), "{}", message);
        Ok(())
    }

    #[test]
    #[ignore]
    fn large_write() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = test_folder("large-write")?;
        let file: FilePath = folder.clone_append("large.file").to_file()?;

        // This spans three parts. The prime period shows a part that is missing or out of order.
        let data: Vec<u8> = (0..11 * 1024 * 1024).map(|i| (i % 251) as u8).collect();
        let mut write: WriteOp = file.write()?;
        write.write_all(&data)?;
        write.close()?;
        assert_eq!(file.read_as_vec()?, data);

        folder.delete_files()?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn folder() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = test_folder("folder")?;

        folder_util::list_files(&folder)?;
        folder_util::delete_files(&folder)?;

        Ok(())
    }
}
