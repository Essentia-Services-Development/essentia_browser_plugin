//! Browser plugin implementation.

use crate::{
    config::BrowserConfig,
    consciousness::ConsciousnessLayer,
    errors::BrowserResult,
    parser::HtmlParser,
    renderer::RenderEngine,
    types::{BrowserTab, NavigationState},
};

/// Main browser plugin interface.
pub struct BrowserPlugin {
    config:        BrowserConfig,
    renderer:      RenderEngine,
    consciousness: ConsciousnessLayer,
    tabs:          Vec<BrowserTab>,
    active_tab:    usize,
    next_tab_id:   u64,
}

impl BrowserPlugin {
    /// Create a new browser plugin.
    pub fn new(config: BrowserConfig) -> Self {
        let consciousness = ConsciousnessLayer::new(config.enable_consciousness);

        Self {
            config,
            renderer: RenderEngine::default(),
            consciousness,
            tabs: Vec::new(),
            active_tab: 0,
            next_tab_id: 1,
        }
    }

    /// Get configuration.
    pub fn config(&self) -> &BrowserConfig {
        &self.config
    }

    /// Open a new tab.
    pub fn new_tab(&mut self) -> u64 {
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        self.tabs.push(BrowserTab {
            id:               tab_id,
            url:              String::from("about:blank"),
            title:            String::from("New Tab"),
            navigation_state: NavigationState::Idle,
            document:         None,
        });

        self.active_tab = self.tabs.len() - 1;
        tab_id
    }

    /// Close a tab.
    pub fn close_tab(&mut self, tab_id: u64) -> bool {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            self.tabs.remove(pos);
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    /// Navigate to URL.
    pub fn navigate(&mut self, url: &str) -> BrowserResult<()> {
        if self.tabs.is_empty() {
            self.new_tab();
        }

        let tab = &mut self.tabs[self.active_tab];
        tab.url = url.to_string();
        tab.navigation_state = NavigationState::Loading;

        // In production, would fetch URL content via essentia_net_plugin
        // For now, create empty document
        let html = "<!DOCTYPE html><html><body></body></html>";
        let document = HtmlParser::parse(html, url)?;

        tab.document = Some(document);
        tab.navigation_state = NavigationState::Loaded;

        Ok(())
    }

    /// Get active tab.
    pub fn active_tab(&self) -> Option<&BrowserTab> {
        self.tabs.get(self.active_tab)
    }

    /// Get all tabs.
    pub fn tabs(&self) -> &[BrowserTab] {
        &self.tabs
    }

    /// Resize viewport.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.renderer.resize(width, height);
    }

    /// Get consciousness coherence score.
    pub fn coherence_score(&self) -> f64 {
        self.consciousness.coherence_score()
    }
}

impl Default for BrowserPlugin {
    fn default() -> Self {
        Self::new(BrowserConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = BrowserPlugin::default();
        assert!(plugin.tabs().is_empty());
    }

    #[test]
    fn test_new_tab() {
        let mut plugin = BrowserPlugin::default();
        let tab_id = plugin.new_tab();
        assert_eq!(tab_id, 1);
        assert_eq!(plugin.tabs().len(), 1);
    }

    #[test]
    fn test_navigate() {
        let mut plugin = BrowserPlugin::default();
        plugin.new_tab();
        let result = plugin.navigate("https://example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_consciousness_enabled() {
        let plugin = BrowserPlugin::default();
        assert!(plugin.coherence_score() > 0.0);
    }
}
