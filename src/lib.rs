//! # Essentia Browser Plugin
//!
//! Pure Rust browser engine with consciousness integration for the Essentia
//! platform.
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

mod config;
mod consciousness;
mod errors;
mod flexforge;
mod parser;
mod plugin;
mod renderer;
mod types;

pub use config::BrowserConfig;
pub use consciousness::ConsciousnessLayer;
pub use errors::{BrowserError, BrowserResult};
pub use flexforge::BrowserFlexForgeIntegration;
pub use parser::HtmlParser;
pub use plugin::BrowserPlugin;
pub use renderer::RenderEngine;
pub use types::{
    BrowserTab, Document, Element, NavigationState, PageMetrics, RenderTree, StyleSheet,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = BrowserConfig::default();
        assert!(config.enable_javascript);
    }
}
