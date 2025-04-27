use file_storage::FilePath;
use std::error::Error;

#[allow(dead_code)]
pub fn test_write(file: &FilePath) -> Result<(), Box<dyn Error>> {
    test_write_slice(file)?;

    Ok(())
}

fn test_write_slice(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_str("Hello, World!")?;
    assert!(file.exists()?);
    assert_eq!(file.read_as_string()?, "Hello, World!");

    Ok(())
}
