use crate::FilePath;

#[test]
fn read_write_exists_delete() -> Result<(), Box<dyn std::error::Error>> {
    let file: FilePath = FilePath::temp()?;
    assert!(!file.exists()?);

    assert!(file.write_str_if_not_exists("Hello, World!")?);
    assert!(file.exists()?);

    assert!(!file.write_data_if_not_exists("Goodbye, World!")?);
    assert!(file.exists()?);

    assert!(file.delete_if_exists()?);
    assert!(!file.exists()?);

    assert!(!file.delete_if_exists()?);

    Ok(())
}
