use file_storage::{FilePath, ReadOp, WriteOp};
use std::error::Error;
use std::io::{Read, Write};

// Tests the operations on the `file`.
pub fn test_file(
    file: &FilePath,
    test_delete_if_exists: bool,
    test_write: bool,
) -> Result<(), Box<dyn Error>> {
    exists(&file)?;
    delete(&file, test_delete_if_exists)?;
    read(&file)?;
    read_string(&file)?;
    read_vec(&file)?;
    if test_write {
        write(&file)?;
        write_if_not_exists(&file)?;
    }
    write_data(&file)?;
    Ok(())
}

/// Tests the `exists` function on the `file`.
fn exists(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_empty()?;
    assert!(file.exists()?);

    Ok(())
}

/// Tests the `delete` functions on the `file`.
fn delete(file: &FilePath, delete_if_exists: bool) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    assert!(!file.exists()?);
    file.write_empty()?;
    assert!(file.exists()?);

    file.delete()?;
    assert!(!file.exists()?);

    if delete_if_exists {
        assert!(!file.delete_if_exists()?);
        file.write_empty()?;
        assert!(file.delete_if_exists()?);
        assert!(!file.exists()?);
    }

    Ok(())
}

/// Tests the `read` functions on the `file`.
fn read(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    assert!(file.read_if_exists()?.is_none());
    file.write_data(s)?;

    let mut read: ReadOp = file.read()?;
    for b in s.as_bytes() {
        let mut buffer: &mut [u8] = &mut [0u8];
        read.read_exact(&mut buffer)?;
        assert_eq!(*b, buffer[0]);
    }
    assert_eq!(read.read(&mut [0u8])?, 0);

    Ok(())
}

/// Tests the `read_string` functions on the `file`.
fn read_string(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    // assert!(file.read_as_string_if_exists()?.is_none());
    file.write_data(s)?;
    assert_eq!(file.read_as_string_if_exists()?.as_deref(), Some(s));

    let mut result: String = String::default();
    file.read_to_string(&mut result)?;
    assert_eq!(result, s);

    Ok(())
}

/// Tests the `read_vec` functions on the `file`.
fn read_vec(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    assert!(file.read_as_vec_if_exists()?.is_none());
    file.write_data(s)?;
    assert_eq!(file.read_as_vec()?, s.as_bytes());

    let mut result: Vec<u8> = Vec::default();
    file.read_to_vec(&mut result)?;
    assert_eq!(result, s.as_bytes());

    Ok(())
}

/// Test the `write` function on the `file`.
fn write(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    let mut write: WriteOp = file.write()?;
    for b in s.as_bytes() {
        write.write_all(&[*b])?;
    }
    write.close()?;
    assert_eq!(file.read_as_string()?, s);

    Ok(())
}

/// Test the `write_if_not_exists` function on the `file`.
fn write_if_not_exists(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    let write: Option<WriteOp> = file.write_if_not_exists()?;
    assert!(write.is_some());
    let mut write: WriteOp = write.unwrap();
    for b in s.as_bytes() {
        write.write_all(&[*b])?;
    }
    write.close()?;
    assert_eq!(file.read_as_string()?, s);

    assert!(file.write_if_not_exists()?.is_none());

    Ok(())
}

/// Test the `write_data` function on the `file`.
fn write_data(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let s: &str = "Hello, World!";

    file.write_data(s)?;
    assert_eq!(file.read_as_string()?, s);

    Ok(())
}
