use file_storage::FilePath;
use std::error::Error;

#[allow(dead_code)]
pub fn test_read(file: &FilePath) -> Result<(), Box<dyn Error>> {
    test_read_to_vec(file)?;
    test_read_to_string(file)?;

    Ok(())
}

fn test_read_to_vec(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let mut vec: Vec<u8> = vec![];
    assert_eq!(file.read_to_vec_if_exists(&mut vec)?, None);

    let vec: Vec<u8> = vec![4, 5, 6];
    file.write_slice(vec.as_slice())?;

    let mut vec: Vec<u8> = vec![1, 2, 3];
    assert_eq!(file.read_to_vec_if_exists(&mut vec)?, Some(3));
    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);

    Ok(())
}

fn test_read_to_string(file: &FilePath) -> Result<(), Box<dyn Error>> {
    file.delete()?;

    let mut string: String = String::default();
    assert_eq!(file.read_to_string_if_exists(&mut string)?, None);

    let string: String = ", World!".to_string();
    file.write_slice(string.as_bytes())?;

    let mut string: String = "Hello".to_string();
    assert_eq!(file.read_to_string_if_exists(&mut string)?, Some(8));
    assert_eq!(string, "Hello, World!");

    Ok(())
}
