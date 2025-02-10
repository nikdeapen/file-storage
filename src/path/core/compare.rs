use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use crate::path::StoragePath;

impl Ord for StoragePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for StoragePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path().partial_cmp(other.path())
    }
}

impl Eq for StoragePath {}

impl PartialEq for StoragePath {
    fn eq(&self, other: &Self) -> bool {
        self.path().eq(other.path())
    }
}

impl Hash for StoragePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state)
    }
}
