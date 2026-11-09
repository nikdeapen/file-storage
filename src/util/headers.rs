use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A set of HTTP headers.
///
/// # Names
/// - Names ignore letter case, like in HTTP. Each keeps the case it was added with.
/// - A set holds one header per name. Adding a name again replaces the header and its case.
/// - Names and values are not checked. R2 may reject an invalid one when the file is written.
///
/// # Cloudflare R2
/// - `Content-Type`, `Cache-Control`, `Content-Encoding`, `Content-Disposition` and
///   `Content-Language` become the object's own headers.
/// - Any other header is stored as custom metadata. R2 receives it as `x-amz-meta-{name}`.
#[must_use]
#[derive(Clone, Debug, Default)]
pub struct Headers {
    /// The headers sorted by name. Names are compared ignoring case and appear once.
    entries: Vec<(String, String)>,
}

impl Headers {
    //! Names

    /// Compares the header names, ignoring letter case.
    fn cmp_names(&self, a: &str, b: &str) -> Ordering {
        let a = a.bytes().map(|byte| byte.to_ascii_lowercase());
        let b = b.bytes().map(|byte| byte.to_ascii_lowercase());
        a.cmp(b)
    }
}

impl Headers {
    //! Construction

    /// Creates an empty set of headers.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Ord for Headers {
    fn cmp(&self, other: &Self) -> Ordering {
        for ((a, x), (b, y)) in self.iter().zip(other.iter()) {
            let order: Ordering = self.cmp_names(a, b).then_with(|| x.cmp(y));
            if order.is_ne() {
                return order;
            }
        }
        self.len().cmp(&other.len())
    }
}

impl PartialOrd for Headers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Headers {}

impl PartialEq for Headers {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|((a, x), (b, y))| a.eq_ignore_ascii_case(b) && x == y)
    }
}

impl Hash for Headers {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for (name, value) in self.iter() {
            name.to_ascii_lowercase().hash(state);
            value.hash(state);
        }
    }
}

impl Headers {
    //! Properties

    /// Checks if there are no headers.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Gets the number of headers.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Gets the value of the header with the `name`. (case-insensitive)
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries
            .binary_search_by(|(n, _)| self.cmp_names(n, name))
            .ok()
            .map(|index| self.entries[index].1.as_str())
    }

    /// Iterates over the headers in order of their names.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(n, v)| (n.as_str(), v.as_str()))
    }
}

impl Headers {
    //! Add

    /// Adds the header.
    ///
    /// # Note
    /// - A header with the same name, in any letter case, is replaced.
    pub fn add_header<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<String>,
    {
        let name: String = name.into();
        let value: String = value.into();
        match self
            .entries
            .binary_search_by(|(n, _)| self.cmp_names(n, &name))
        {
            Ok(index) => self.entries[index] = (name, value),
            Err(index) => self.entries.insert(index, (name, value)),
        }
    }

    /// Adds the header.
    ///
    /// # Note
    /// - A header with the same name, in any letter case, is replaced.
    pub fn with_header<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.add_header(name, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use crate::Headers;

    fn hash_of(headers: &Headers) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        headers.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn empty() {
        let headers: Headers = Headers::new();
        assert!(headers.is_empty());
        assert_eq!(headers.len(), 0);
        assert_eq!(headers.get("Content-Type"), None);
    }

    #[test]
    fn get_ignores_case() {
        let headers: Headers = Headers::new().with_header("Content-Type", "text/plain");
        assert_eq!(headers.get("content-type"), Some("text/plain"));
        assert_eq!(headers.get("CONTENT-TYPE"), Some("text/plain"));
    }

    #[test]
    fn adding_a_name_again_replaces_it() {
        let headers: Headers = Headers::new().with_header("A", "1").with_header("a", "2");
        let entries: Vec<(&str, &str)> = headers.iter().collect();
        assert_eq!(entries, vec![("a", "2")]);
    }

    #[test]
    fn iter_keeps_case_and_sorts_ignoring_case() {
        let mut headers: Headers = Headers::new();
        headers.add_header("B", "2");
        headers.add_header("a", "1");
        let entries: Vec<(&str, &str)> = headers.iter().collect();
        assert_eq!(entries, vec![("a", "1"), ("B", "2")]);
    }

    #[test]
    fn eq_ord_and_hash_ignore_case_and_order() {
        let first: Headers = Headers::new().with_header("A", "1").with_header("b", "2");
        let second: Headers = Headers::new().with_header("B", "2").with_header("a", "1");
        assert_eq!(first, second);
        assert_eq!(first.cmp(&second), Ordering::Equal);
        assert_eq!(hash_of(&first), hash_of(&second));
    }

    #[test]
    fn eq_compares_values_exactly() {
        let first: Headers = Headers::new().with_header("A", "x");
        let second: Headers = Headers::new().with_header("A", "X");
        assert_ne!(first, second);
    }

    #[test]
    fn ord_compares_names_ignoring_case_then_values_then_length() {
        let a1: Headers = Headers::new().with_header("a", "1");
        let a2: Headers = Headers::new().with_header("A", "2");
        let b1: Headers = Headers::new().with_header("B", "1");
        let a1_b1: Headers = a1.clone().with_header("b", "1");

        assert!(a1 < b1);
        assert!(a1 < a2);
        assert!(a1 < a1_b1);
        assert!(a1_b1 < a2);
        assert!(Headers::new() < a1);
    }
}
