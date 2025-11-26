//! Browser plugin type definitions.

/// HTML document representation.
#[derive(Debug, Clone)]
pub struct Document {
    /// Document title.
    pub title: String,
    /// Root element.
    pub root: Element,
    /// Document URL.
    pub url: String,
}

/// HTML element.
#[derive(Debug, Clone)]
pub struct Element {
    /// Tag name.
    pub tag: String,
    /// Element attributes.
    pub attributes: Vec<(String, String)>,
    /// Child elements.
    pub children: Vec<Element>,
    /// Text content.
    pub text_content: Option<String>,
}

impl Element {
    /// Create a new element.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            text_content: None,
        }
    }

    /// Add an attribute.
    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((name.into(), value.into()));
        self
    }

    /// Add a child element.
    pub fn with_child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    /// Set text content.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text_content = Some(text.into());
        self
    }
}

/// CSS stylesheet.
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    /// CSS rules.
    pub rules: Vec<CssRule>,
}

/// CSS rule.
#[derive(Debug, Clone)]
pub struct CssRule {
    /// Selector.
    pub selector: String,
    /// Declarations.
    pub declarations: Vec<(String, String)>,
}

/// Render tree for layout.
#[derive(Debug, Clone)]
pub struct RenderTree {
    /// Root render node.
    pub root: RenderNode,
}

/// Render node.
#[derive(Debug, Clone)]
pub struct RenderNode {
    /// Associated element.
    pub element: Element,
    /// Computed styles.
    pub computed_style: ComputedStyle,
    /// Layout box.
    pub layout: LayoutBox,
    /// Child nodes.
    pub children: Vec<RenderNode>,
}

/// Computed CSS style.
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    /// Display mode.
    pub display: Display,
    /// Width in pixels.
    pub width: Option<f32>,
    /// Height in pixels.
    pub height: Option<f32>,
    /// Background color.
    pub background_color: Color,
    /// Text color.
    pub color: Color,
}

/// Display mode.
#[derive(Debug, Clone, Copy, Default)]
pub enum Display {
    #[default]
    Block,
    Inline,
    InlineBlock,
    Flex,
    None,
}

/// RGBA color.
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}

/// Layout box dimensions.
#[derive(Debug, Clone, Default)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Browser tab.
#[derive(Debug, Clone)]
pub struct BrowserTab {
    /// Tab ID.
    pub id: u64,
    /// Current URL.
    pub url: String,
    /// Page title.
    pub title: String,
    /// Navigation state.
    pub navigation_state: NavigationState,
    /// Loaded document.
    pub document: Option<Document>,
}

/// Navigation state.
#[derive(Debug, Clone, Copy, Default)]
pub enum NavigationState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

/// Page performance metrics.
#[derive(Debug, Clone, Default)]
pub struct PageMetrics {
    /// Time to first byte (ms).
    pub ttfb: f64,
    /// DOM content loaded (ms).
    pub dom_content_loaded: f64,
    /// Page load complete (ms).
    pub load_complete: f64,
    /// First contentful paint (ms).
    pub first_contentful_paint: f64,
}
