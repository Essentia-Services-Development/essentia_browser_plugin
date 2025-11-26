//! Browser plugin configuration.

/// Configuration for the browser plugin.
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Enable JavaScript execution.
    pub enable_javascript:    bool,
    /// Enable image loading.
    pub enable_images:        bool,
    /// Enable CSS.
    pub enable_css:           bool,
    /// Maximum concurrent connections.
    pub max_connections:      usize,
    /// User agent string.
    pub user_agent:           String,
    /// Enable consciousness pattern recognition.
    pub enable_consciousness: bool,
    /// Maximum memory usage (bytes).
    pub max_memory:           usize,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            enable_javascript:    true,
            enable_images:        true,
            enable_css:           true,
            max_connections:      6,
            user_agent:           String::from("EssentiaBrowser/1.0"),
            enable_consciousness: true,
            max_memory:           512 * 1024 * 1024, // 512 MB
        }
    }
}
