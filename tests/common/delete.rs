use file_storage::FilePath;
use std::error::Error;

#[allow(dead_code)]
pub fn test_delete(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    file.write_str("Hello, World!")?;
    assert!(file.exists()?);
    file.delete()?;
    assert!(!file.exists()?);

    Ok(())
}

#[allow(dead_code)]
pub fn test_delete_if_exist(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    assert!(!file.delete_if_exists()?);
    assert!(!file.exists()?);

    file.write_str("Hello, World!")?;
    assert!(file.exists()?);
    assert!(file.delete_if_exists()?);
    assert!(!file.exists()?);

    Ok(())
}
