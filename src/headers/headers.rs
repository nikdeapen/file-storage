/// A set of HTTP headers.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Headers {
    entries: Vec<(String, String)>,
}

impl Headers {
    //! Construction

    /// Creates an empty set of headers.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Headers {
    //! Headers

    /// Checks if there are no headers.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Gets the number of headers.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Adds the header.
    pub fn add_header<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.entries.push((name.into(), value.into()));
    }

    /// Adds the header.
    #[must_use]
    pub fn with_header<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.add_header(name, value);
        self
    }

    /// Gets the first value for the header with the `name`. (case-insensitive)
    pub fn get(&self, name: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// Iterates over the headers.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(n, v)| (n.as_str(), v.as_str()))
    }
}
