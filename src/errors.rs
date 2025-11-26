//! Browser plugin error types.

use core::fmt;

/// Browser operation errors.
#[derive(Debug)]
pub enum BrowserError {
    /// HTML parsing error.
    Parse(String),
    /// CSS parsing error.
    Css(String),
    /// JavaScript error.
    Script(String),
    /// Rendering error.
    Render(String),
    /// Network error.
    Network(String),
    /// Navigation error.
    Navigation(String),
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::Css(msg) => write!(f, "CSS error: {msg}"),
            Self::Script(msg) => write!(f, "Script error: {msg}"),
            Self::Render(msg) => write!(f, "Render error: {msg}"),
            Self::Network(msg) => write!(f, "Network error: {msg}"),
            Self::Navigation(msg) => write!(f, "Navigation error: {msg}"),
        }
    }
}

/// Result type for browser operations.
pub type BrowserResult<T> = Result<T, BrowserError>;
