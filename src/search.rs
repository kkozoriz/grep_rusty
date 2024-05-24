/// Trait defining a search option.
///
/// This trait provides a common interface for various search options, allowing
/// them to be used interchangeably in search configurations.
pub trait SearchOption: Send + Sync {
    /// Checks if a line matches the search query.
    ///
    /// # Arguments
    ///
    /// * `line` - The line to search within.
    /// * `query` - The query string to search for.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the line matches the query, `false` otherwise.
    fn matches(&self, line: &str, query: &str) -> bool;
}

/// Struct representing a case-insensitive search option.
pub struct CaseInsensitive;

/// Struct representing a case-sensitive search option.
pub struct CaseSensitive;

/// Struct representing an inverted search option.
pub struct InvertMatch<T: SearchOption> {
    /// The inner search option to invert.
    pub inner: T,
}

/// Struct representing a search configuration.
pub struct SearchConfig {
    /// The list of search options in the configuration.
    pub configs: Vec<Box<dyn SearchOption>>,
}

impl SearchOption for CaseSensitive {
    fn matches(&self, line: &str, query: &str) -> bool {
        line.contains(query)
    }
}

impl SearchOption for CaseInsensitive {
    fn matches(&self, line: &str, query: &str) -> bool {
        line.to_lowercase().contains(&query.to_lowercase())
    }
}

impl SearchOption for SearchConfig {
    fn matches(&self, line: &str, query: &str) -> bool {
        self.configs.iter().all(|s| s.matches(line, query))
    }
}

impl<T: SearchOption> SearchOption for InvertMatch<T> {
    fn matches(&self, line: &str, query: &str) -> bool {
        !self.inner.matches(line, query)
    }
}

impl SearchConfig {
    /// Creates a new empty search configuration.
    pub fn new() -> Self {
        Self { configs: vec![] }
    }

    /// Adds a search option to the configuration.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The search option to add.
    pub fn add_config(&mut self, strategy: Box<dyn SearchOption>) {
        self.configs.push(strategy);
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self::new()
    }
}