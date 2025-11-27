//! FlexForge Universal Editor Integration for Browser Plugin
//!
//! Provides comprehensive browser integration within FlexForge including:
//! - Tab management and navigation
//! - Real-time page rendering streaming
//! - Privacy and security configuration
//! - Developer tools integration
//! - AI-assisted web interaction
//!
//! ## Features
//!
//! - Multi-tab browser with synchronized state
//! - RSP+++ streaming for 60fps page rendering
//! - Integrated developer tools panel
//! - Privacy modes: Standard, Strict, Private
//! - AI content analysis and summarization

#![allow(dead_code)]
#![allow(clippy::collapsible_if)]

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use essentia_traits::plugin_contracts::flexforge_integration::{
    ConfigField, ConfigSchema, EditorAction, EditorPresentable, FlexForgeCapability,
    FlexForgeIntegration, FlexForgePanelCategory, FlexForgePanelInfo, StreamingCapable,
    UiConfigurable,
};

/// Browser configuration for FlexForge panel
#[derive(Debug, Clone)]
pub struct BrowserFlexForgeConfig {
    // Privacy & Security
    pub enable_javascript:     bool,
    pub enable_cookies:        bool,
    pub privacy_mode:          String,
    pub block_trackers:        bool,
    pub https_only:            bool,
    // Performance
    pub max_tabs:              u32,
    pub cache_size_mb:         u32,
    pub preload_links:         bool,
    pub hardware_acceleration: bool,
    // Appearance
    pub user_agent:            String,
    pub default_zoom:          u32,
    pub dark_mode:             bool,
    // AI Features
    pub ai_content_summary:    bool,
    pub ai_translation:        bool,
    pub ai_reading_mode:       bool,
}

impl Default for BrowserFlexForgeConfig {
    fn default() -> Self {
        Self {
            enable_javascript:     true,
            enable_cookies:        true,
            privacy_mode:          "standard".to_string(),
            block_trackers:        true,
            https_only:            false,
            max_tabs:              50,
            cache_size_mb:         256,
            preload_links:         true,
            hardware_acceleration: true,
            user_agent:            "EssentiaBrowser/1.0 (FlexForge)".to_string(),
            default_zoom:          100,
            dark_mode:             false,
            ai_content_summary:    true,
            ai_translation:        false,
            ai_reading_mode:       false,
        }
    }
}

/// Browser tab state.
#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id:             u64,
    pub url:            String,
    pub title:          String,
    pub favicon:        Option<String>,
    pub loading:        bool,
    pub can_go_back:    bool,
    pub can_go_forward: bool,
    pub zoom_level:     u32,
    pub pinned:         bool,
}

impl Default for BrowserTab {
    fn default() -> Self {
        Self {
            id:             0,
            url:            String::from("about:blank"),
            title:          String::from("New Tab"),
            favicon:        None,
            loading:        false,
            can_go_back:    false,
            can_go_forward: false,
            zoom_level:     100,
            pinned:         false,
        }
    }
}

/// Browser metrics for monitoring.
#[derive(Debug, Clone, Default)]
pub struct BrowserMetrics {
    pub open_tabs:        u32,
    pub memory_usage_mb:  u64,
    pub network_requests: u64,
    pub blocked_trackers: u64,
    pub render_fps:       f32,
    pub page_load_ms:     u64,
}

/// FlexForge integration for the Browser plugin
#[derive(Debug)]
pub struct BrowserFlexForgeIntegration {
    config:         Arc<Mutex<BrowserFlexForgeConfig>>,
    metrics:        Arc<Mutex<BrowserMetrics>>,
    tabs:           Arc<Mutex<HashMap<u64, BrowserTab>>>,
    active_tab_id:  Option<u64>,
    next_tab_id:    u64,
    stream_active:  bool,
    stream_id:      Option<u64>,
    next_stream_id: u64,
    devtools_open:  bool,
}

impl BrowserFlexForgeIntegration {
    /// Create a new FlexForge integration instance
    #[must_use]
    pub fn new() -> Self {
        let mut tabs = HashMap::new();
        let initial_tab = BrowserTab { id: 1, ..Default::default() };
        tabs.insert(1, initial_tab);

        Self {
            config:         Arc::new(Mutex::new(BrowserFlexForgeConfig::default())),
            metrics:        Arc::new(Mutex::new(BrowserMetrics::default())),
            tabs:           Arc::new(Mutex::new(tabs)),
            active_tab_id:  Some(1),
            next_tab_id:    2,
            stream_active:  false,
            stream_id:      None,
            next_stream_id: 1,
            devtools_open:  false,
        }
    }

    fn config(&self) -> BrowserFlexForgeConfig {
        self.config.lock().map(|c| c.clone()).unwrap_or_default()
    }

    fn set_config(&self, config: BrowserFlexForgeConfig) {
        if let Ok(mut guard) = self.config.lock() {
            *guard = config;
        }
    }

    /// Creates a new tab and returns its ID.
    pub fn create_tab(&mut self, url: Option<&str>) -> u64 {
        let tab_id = self.next_tab_id;
        self.next_tab_id = self.next_tab_id.wrapping_add(1);

        let tab = BrowserTab {
            id: tab_id,
            url: url.unwrap_or("about:blank").to_string(),
            loading: url.is_some(),
            ..Default::default()
        };

        if let Ok(mut tabs) = self.tabs.lock() {
            tabs.insert(tab_id, tab);
        }

        self.active_tab_id = Some(tab_id);
        self.update_tab_metrics();
        tab_id
    }

    /// Closes a tab by ID.
    pub fn close_tab(&mut self, tab_id: u64) -> Result<(), String> {
        if let Ok(mut tabs) = self.tabs.lock() {
            if tabs.len() <= 1 {
                return Err("Cannot close last tab".to_string());
            }
            tabs.remove(&tab_id);

            // Switch to another tab if this was active
            if self.active_tab_id == Some(tab_id) {
                self.active_tab_id = tabs.keys().next().copied();
            }
        }
        self.update_tab_metrics();
        Ok(())
    }

    /// Navigates the active tab to a URL.
    pub fn navigate(&mut self, url: &str) -> Result<(), String> {
        let tab_id = self.active_tab_id.ok_or("No active tab")?;

        if let Ok(mut tabs) = self.tabs.lock() {
            if let Some(tab) = tabs.get_mut(&tab_id) {
                tab.url = url.to_string();
                tab.loading = true;
                tab.can_go_back = true;
            }
        }
        Ok(())
    }

    /// Returns panel info with full capabilities.
    #[must_use]
    pub fn panel_info(&self) -> FlexForgePanelInfo {
        FlexForgePanelInfo {
            id:           self.panel_id().to_string(),
            name:         self.display_name().to_string(),
            category:     self.category(),
            icon:         self.icon_glyph().map(String::from),
            priority:     self.priority(),
            capabilities: vec![
                FlexForgeCapability::Configuration,
                FlexForgeCapability::Editor,
                FlexForgeCapability::Streaming,
                FlexForgeCapability::Visualization,
            ],
        }
    }

    fn update_tab_metrics(&self) {
        if let (Ok(tabs), Ok(mut metrics)) = (self.tabs.lock(), self.metrics.lock()) {
            metrics.open_tabs = tabs.len() as u32;
        }
    }

    fn next_stream(&mut self) -> u64 {
        let id = self.next_stream_id;
        self.next_stream_id = self.next_stream_id.wrapping_add(1);
        id
    }
}

impl Default for BrowserFlexForgeIntegration {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// FlexForge Integration Trait
// ============================================================================

impl FlexForgeIntegration for BrowserFlexForgeIntegration {
    fn panel_id(&self) -> &str {
        "essentia_browser_plugin"
    }

    fn category(&self) -> FlexForgePanelCategory {
        FlexForgePanelCategory::Media
    }

    fn display_name(&self) -> &str {
        "Browser"
    }

    fn icon_glyph(&self) -> Option<&str> {
        Some("\u{E774}") // Globe/Web icon
    }

    fn priority(&self) -> u32 {
        2 // High priority in Media category
    }

    fn on_panel_activate(&mut self) {
        // Start rendering stream when panel becomes visible
        if !self.stream_active {
            let _ = self.start_stream();
        }
    }

    fn on_panel_deactivate(&mut self) {
        // Stop streaming when panel is hidden
        if let Some(id) = self.stream_id {
            let _ = self.stop_stream(id);
        }
    }

    fn on_refresh(&mut self) -> bool {
        // Refresh if any tab is loading
        if let Ok(tabs) = self.tabs.lock() {
            tabs.values().any(|t| t.loading)
        } else {
            false
        }
    }
}

// ============================================================================
// UI Configurable Trait
// ============================================================================

impl UiConfigurable for BrowserFlexForgeIntegration {
    fn config_schema(&self) -> ConfigSchema {
        ConfigSchema::new()
            // Privacy & Security
            .with_field(
                ConfigField::toggle("enable_javascript", "Enable JavaScript", true)
                    .with_description("Allow JavaScript execution on pages")
                    .with_group("Privacy & Security"),
            )
            .with_field(
                ConfigField::toggle("enable_cookies", "Enable Cookies", true)
                    .with_description("Allow websites to store cookies")
                    .with_group("Privacy & Security"),
            )
            .with_field(
                ConfigField::select(
                    "privacy_mode",
                    "Privacy Mode",
                    vec![
                        "standard".to_string(),
                        "strict".to_string(),
                        "private".to_string(),
                    ],
                )
                .with_description("Privacy protection level")
                .with_group("Privacy & Security"),
            )
            .with_field(
                ConfigField::toggle("block_trackers", "Block Trackers", true)
                    .with_description("Block known tracking scripts")
                    .with_group("Privacy & Security"),
            )
            .with_field(
                ConfigField::toggle("https_only", "HTTPS Only Mode", false)
                    .with_description("Only connect to secure websites")
                    .with_group("Privacy & Security"),
            )
            // Performance
            .with_field(
                ConfigField::number("max_tabs", "Max Tabs", 50.0, 1.0, 100.0)
                    .with_description("Maximum number of open tabs")
                    .with_group("Performance"),
            )
            .with_field(
                ConfigField::number("cache_size_mb", "Cache Size (MB)", 256.0, 0.0, 2048.0)
                    .with_description("Browser cache size limit")
                    .with_group("Performance"),
            )
            .with_field(
                ConfigField::toggle("preload_links", "Preload Links", true)
                    .with_description("Preload hovered links for faster navigation")
                    .with_group("Performance"),
            )
            .with_field(
                ConfigField::toggle("hardware_acceleration", "Hardware Acceleration", true)
                    .with_description("Use GPU for rendering")
                    .with_group("Performance"),
            )
            // Appearance
            .with_field(
                ConfigField::text("user_agent", "User Agent")
                    .with_description("Browser identification string")
                    .with_group("Appearance"),
            )
            .with_field(
                ConfigField::number("default_zoom", "Default Zoom (%)", 100.0, 25.0, 500.0)
                    .with_description("Default page zoom level")
                    .with_group("Appearance"),
            )
            .with_field(
                ConfigField::toggle("dark_mode", "Dark Mode", false)
                    .with_description("Force dark mode on websites")
                    .with_group("Appearance"),
            )
            // AI Features
            .with_field(
                ConfigField::toggle("ai_content_summary", "AI Page Summary", true)
                    .with_description("Generate AI summaries of page content")
                    .with_group("AI Features"),
            )
            .with_field(
                ConfigField::toggle("ai_translation", "AI Translation", false)
                    .with_description("Auto-translate foreign language pages")
                    .with_group("AI Features"),
            )
            .with_field(
                ConfigField::toggle("ai_reading_mode", "AI Reading Mode", false)
                    .with_description("Simplify pages for easier reading")
                    .with_group("AI Features"),
            )
    }

    fn on_config_changed(&mut self, key: &str, value: &str) -> Result<(), String> {
        let mut config = self.config();
        match key {
            "enable_javascript" => config.enable_javascript = value == "true",
            "enable_cookies" => config.enable_cookies = value == "true",
            "privacy_mode" => config.privacy_mode = value.to_string(),
            "block_trackers" => config.block_trackers = value == "true",
            "https_only" => config.https_only = value == "true",
            "max_tabs" => {
                config.max_tabs = value.parse().map_err(|_| "Invalid number")?;
            },
            "cache_size_mb" => {
                config.cache_size_mb = value.parse().map_err(|_| "Invalid number")?;
            },
            "preload_links" => config.preload_links = value == "true",
            "hardware_acceleration" => config.hardware_acceleration = value == "true",
            "user_agent" => config.user_agent = value.to_string(),
            "default_zoom" => {
                let zoom: u32 = value.parse().map_err(|_| "Invalid number")?;
                if !(25..=500).contains(&zoom) {
                    return Err("Zoom must be between 25% and 500%".to_string());
                }
                config.default_zoom = zoom;
            },
            "dark_mode" => config.dark_mode = value == "true",
            "ai_content_summary" => config.ai_content_summary = value == "true",
            "ai_translation" => config.ai_translation = value == "true",
            "ai_reading_mode" => config.ai_reading_mode = value == "true",
            _ => return Err(format!("Unknown key: {}", key)),
        }
        self.set_config(config);
        Ok(())
    }

    fn apply_config(&mut self, config: &[(String, String)]) -> Result<(), String> {
        for (key, value) in config {
            self.on_config_changed(key, value)?;
        }
        Ok(())
    }

    fn get_current_config(&self) -> Vec<(String, String)> {
        let config = self.config();
        vec![
            (
                "enable_javascript".to_string(),
                config.enable_javascript.to_string(),
            ),
            (
                "enable_cookies".to_string(),
                config.enable_cookies.to_string(),
            ),
            ("privacy_mode".to_string(), config.privacy_mode),
            (
                "block_trackers".to_string(),
                config.block_trackers.to_string(),
            ),
            ("https_only".to_string(), config.https_only.to_string()),
            ("max_tabs".to_string(), config.max_tabs.to_string()),
            (
                "cache_size_mb".to_string(),
                config.cache_size_mb.to_string(),
            ),
            (
                "preload_links".to_string(),
                config.preload_links.to_string(),
            ),
            (
                "hardware_acceleration".to_string(),
                config.hardware_acceleration.to_string(),
            ),
            ("user_agent".to_string(), config.user_agent),
            ("default_zoom".to_string(), config.default_zoom.to_string()),
            ("dark_mode".to_string(), config.dark_mode.to_string()),
            (
                "ai_content_summary".to_string(),
                config.ai_content_summary.to_string(),
            ),
            (
                "ai_translation".to_string(),
                config.ai_translation.to_string(),
            ),
            (
                "ai_reading_mode".to_string(),
                config.ai_reading_mode.to_string(),
            ),
        ]
    }

    fn reset_to_defaults(&mut self) {
        self.set_config(BrowserFlexForgeConfig::default());
    }
}

// ============================================================================
// Editor Presentable Trait
// ============================================================================

impl EditorPresentable for BrowserFlexForgeIntegration {
    fn editor_type(&self) -> &str {
        "browser_tabs"
    }

    fn supported_content_types(&self) -> Vec<String> {
        vec![
            String::from("text/html"),
            String::from("application/xhtml+xml"),
            String::from("text/plain"),
            String::from("application/pdf"),
            String::from("image/*"),
            String::from("essentia/browser-session"),
        ]
    }

    fn load_content(&mut self, content_id: &str, content_type: &str) -> Result<(), String> {
        match content_type {
            "essentia/browser-session" => {
                // Load saved browser session
                // content_id would be a session file path
                Ok(())
            },
            _ => {
                // Navigate to URL
                self.navigate(content_id)
            },
        }
    }

    fn save_content(&self) -> Result<String, String> {
        // Serialize current tabs as session
        if let Ok(tabs) = self.tabs.lock() {
            let urls: Vec<&str> = tabs.values().map(|t| t.url.as_str()).collect();
            Ok(urls.join(";"))
        } else {
            Err("Failed to access tabs".to_string())
        }
    }

    fn has_unsaved_changes(&self) -> bool {
        // Browser doesn't have traditional "unsaved" state
        // Could track form data or pinned tabs
        false
    }

    fn get_toolbar_actions(&self) -> Vec<EditorAction> {
        let has_active = self.active_tab_id.is_some();
        let can_go_back = self
            .tabs
            .lock()
            .ok()
            .and_then(|tabs| self.active_tab_id.and_then(|id| tabs.get(&id).map(|t| t.can_go_back)))
            .unwrap_or(false);
        let can_go_forward = self
            .tabs
            .lock()
            .ok()
            .and_then(|tabs| {
                self.active_tab_id.and_then(|id| tabs.get(&id).map(|t| t.can_go_forward))
            })
            .unwrap_or(false);

        vec![
            EditorAction {
                id:       String::from("browser_back"),
                label:    String::from("Back"),
                icon:     String::from("\u{E72B}"),
                shortcut: Some(String::from("Alt+Left")),
                enabled:  can_go_back,
            },
            EditorAction {
                id:       String::from("browser_forward"),
                label:    String::from("Forward"),
                icon:     String::from("\u{E72A}"),
                shortcut: Some(String::from("Alt+Right")),
                enabled:  can_go_forward,
            },
            EditorAction {
                id:       String::from("browser_refresh"),
                label:    String::from("Refresh"),
                icon:     String::from("\u{E72C}"),
                shortcut: Some(String::from("F5")),
                enabled:  has_active,
            },
            EditorAction {
                id:       String::from("browser_home"),
                label:    String::from("Home"),
                icon:     String::from("\u{E80F}"),
                shortcut: Some(String::from("Alt+Home")),
                enabled:  true,
            },
            EditorAction {
                id:       String::from("browser_new_tab"),
                label:    String::from("New Tab"),
                icon:     String::from("\u{E710}"),
                shortcut: Some(String::from("Ctrl+T")),
                enabled:  true,
            },
            EditorAction {
                id:       String::from("browser_close_tab"),
                label:    String::from("Close Tab"),
                icon:     String::from("\u{E711}"),
                shortcut: Some(String::from("Ctrl+W")),
                enabled:  has_active,
            },
            EditorAction {
                id:       String::from("browser_devtools"),
                label:    String::from("Developer Tools"),
                icon:     String::from("\u{E943}"),
                shortcut: Some(String::from("F12")),
                enabled:  has_active,
            },
            EditorAction {
                id:       String::from("browser_ai_summary"),
                label:    String::from("AI Summary"),
                icon:     String::from("\u{E945}"),
                shortcut: Some(String::from("Ctrl+Shift+S")),
                enabled:  has_active && self.config().ai_content_summary,
            },
        ]
    }
}

// ============================================================================
// Streaming Capable Trait
// ============================================================================

impl StreamingCapable for BrowserFlexForgeIntegration {
    fn is_streaming(&self) -> bool {
        self.stream_active
    }

    fn start_stream(&mut self) -> Result<u64, String> {
        if self.stream_active {
            return Err("Stream already active".to_string());
        }

        let stream_id = self.next_stream();
        self.stream_id = Some(stream_id);
        self.stream_active = true;

        Ok(stream_id)
    }

    fn stop_stream(&mut self, stream_id: u64) -> Result<(), String> {
        if !self.stream_active {
            return Err("No active stream".to_string());
        }

        if self.stream_id != Some(stream_id) {
            return Err("Invalid stream ID".to_string());
        }

        self.stream_active = false;
        self.stream_id = None;

        Ok(())
    }

    fn target_fps(&self) -> u32 {
        // Browser rendering targets 60fps
        60
    }

    fn render_frame(&mut self, stream_id: u64, _delta_ms: f64) -> bool {
        if !self.stream_active || self.stream_id != Some(stream_id) {
            return false;
        }

        // Update render metrics
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.render_fps = 60.0; // Would come from actual renderer
        }

        true
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let integration = BrowserFlexForgeIntegration::new();
        let config = integration.config();
        assert!(config.enable_javascript);
        assert!(config.enable_cookies);
        assert_eq!(config.privacy_mode, "standard");
        assert!(config.block_trackers);
        assert!(config.ai_content_summary);
    }

    #[test]
    fn test_panel_info() {
        let integration = BrowserFlexForgeIntegration::new();
        assert_eq!(integration.panel_id(), "essentia_browser_plugin");
        assert_eq!(integration.category(), FlexForgePanelCategory::Media);
        assert_eq!(integration.priority(), 2);
    }

    #[test]
    fn test_privacy_mode_change() {
        let mut integration = BrowserFlexForgeIntegration::new();
        integration.on_config_changed("privacy_mode", "strict").unwrap();
        assert_eq!(integration.config().privacy_mode, "strict");
    }

    #[test]
    fn test_tab_management() {
        let mut integration = BrowserFlexForgeIntegration::new();

        // Initial tab exists
        assert_eq!(integration.active_tab_id, Some(1));

        // Create new tab
        let tab_id = integration.create_tab(Some("https://example.com"));
        assert_eq!(integration.active_tab_id, Some(tab_id));

        // Close tab
        integration.close_tab(tab_id).unwrap();
        assert_ne!(integration.active_tab_id, Some(tab_id));
    }

    #[test]
    fn test_cannot_close_last_tab() {
        let mut integration = BrowserFlexForgeIntegration::new();
        let result = integration.close_tab(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_navigation() {
        let mut integration = BrowserFlexForgeIntegration::new();
        integration.navigate("https://essentia.dev").unwrap();

        if let Ok(tabs) = integration.tabs.lock() {
            let tab = tabs.get(&1).unwrap();
            assert_eq!(tab.url, "https://essentia.dev");
            assert!(tab.loading);
        }
    }

    #[test]
    fn test_streaming_lifecycle() {
        let mut integration = BrowserFlexForgeIntegration::new();

        let stream_id = integration.start_stream().expect("Should start");
        assert!(integration.is_streaming());
        assert_eq!(integration.target_fps(), 60);

        // Render a frame
        assert!(integration.render_frame(stream_id, 16.67));

        integration.stop_stream(stream_id).expect("Should stop");
        assert!(!integration.is_streaming());
    }

    #[test]
    fn test_editor_actions() {
        let integration = BrowserFlexForgeIntegration::new();
        let actions = integration.get_toolbar_actions();

        assert!(!actions.is_empty());
        assert!(actions.iter().any(|a| a.id == "browser_new_tab"));
        assert!(actions.iter().any(|a| a.id == "browser_refresh"));
        assert!(actions.iter().any(|a| a.id == "browser_devtools"));
    }

    #[test]
    fn test_config_schema_groups() {
        let integration = BrowserFlexForgeIntegration::new();
        let schema = integration.config_schema();

        // Check all groups are represented
        let groups: Vec<&str> = schema.fields.iter().filter_map(|f| f.group.as_deref()).collect();

        assert!(groups.contains(&"Privacy & Security"));
        assert!(groups.contains(&"Performance"));
        assert!(groups.contains(&"Appearance"));
        assert!(groups.contains(&"AI Features"));
    }

    #[test]
    fn test_zoom_validation() {
        let mut integration = BrowserFlexForgeIntegration::new();

        // Valid zoom
        assert!(integration.on_config_changed("default_zoom", "150").is_ok());
        assert_eq!(integration.config().default_zoom, 150);

        // Invalid zoom (too low)
        assert!(integration.on_config_changed("default_zoom", "10").is_err());

        // Invalid zoom (too high)
        assert!(integration.on_config_changed("default_zoom", "600").is_err());
    }
}
