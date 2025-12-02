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
//! │        (Pattern Recognition & Coherence)             │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```

// Browser plugin pedantic lint allowances (BROWSER-LINT-STAGING-01)
// HTML/CSS parsing with many string operations and builder patterns
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::unused_self)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::implicit_clone)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::if_not_else)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::unnested_or_patterns)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::match_bool)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::range_plus_one)]
#![allow(clippy::ignored_unit_patterns)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::elidable_lifetime_names)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::format_push_string)]
#![allow(clippy::manual_string_new)]
#![allow(clippy::self_only_used_in_recursion)]

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
