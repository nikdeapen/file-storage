use crate::StoragePath;

impl StoragePath {
    //! Base & Extension

    /// Gets the path base.
    pub fn base(&self) -> &str {
        &self.path()[..self.base_len()]
    }

    /// Gets the path extension.
    pub fn extension(&self) -> &str {
        &self.path()[self.base_len()..]
    }
}
