use crate::StoragePath;

#[allow(clippy::len_without_is_empty)]
impl StoragePath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn len() {
        let path: StoragePath = unsafe { StoragePath::new("/你/好", 1, '/') };
        assert_eq!(path.len(), 8);
    }
}
