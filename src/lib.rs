//! # Essentia Browser Plugin
//!
//! Pure Rust browser engine with consciousness integration for the Essentia platform.
//!
//! ## Features
//!
//! - HTML/CSS parsing and rendering
//! - JavaScript execution (via pure Rust interpreter)
//! - Consciousness-aware browsing patterns
//! - Integration with FFUI rendering pipeline
//! - RSP streaming transport for content delivery
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Browser Plugin                            │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │    HTML     │  │     CSS     │  │    JavaScript       │  │
//! │  │   Parser    │  │   Engine    │  │    Interpreter      │  │
//! │  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
//! │         │                │                     │             │
//! │         ▼                ▼                     ▼             │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │                  Rendering Engine                    │    │
//! │  │          (Layout → Paint → Composite)                │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! │                          │                                   │
//! │                          ▼                                   │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │               Consciousness Layer                    │    │
//! │  │        (Pattern Recognition & Coherence)             │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```

mod types;
mod errors;
mod config;
mod parser;
mod renderer;
mod consciousness;
mod plugin;

pub use types::{
    Document, Element, StyleSheet, RenderTree, BrowserTab,
    NavigationState, PageMetrics,
};
pub use errors::{BrowserError, BrowserResult};
pub use config::BrowserConfig;
pub use parser::HtmlParser;
pub use renderer::RenderEngine;
pub use consciousness::ConsciousnessLayer;
pub use plugin::BrowserPlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = BrowserConfig::default();
        assert!(config.enable_javascript);
    }
}
