use file_storage::FilePath;
use std::error::Error;

#[allow(dead_code)]
pub fn test_exists(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_str("Hello, World!")?;
    assert!(file.exists()?);

    Ok(())
}
