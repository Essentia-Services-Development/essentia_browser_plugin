//! Rendering engine.

use crate::{
    errors::BrowserResult,
    types::{ComputedStyle, Document, LayoutBox, RenderNode, RenderTree},
};

/// Render engine for layout and painting.
pub struct RenderEngine {
    viewport_width:  f32,
    viewport_height: f32,
}

impl RenderEngine {
    /// Create a new render engine.
    pub fn new(width: f32, height: f32) -> Self {
        Self { viewport_width: width, viewport_height: height }
    }

    /// Build render tree from document.
    pub fn build_render_tree(&self, document: &Document) -> BrowserResult<RenderTree> {
        let root_node = self.build_render_node(&document.root, 0.0, 0.0);
        Ok(RenderTree { root: root_node })
    }

    /// Build a single render node.
    fn build_render_node(&self, element: &crate::types::Element, x: f32, y: f32) -> RenderNode {
        let computed_style = ComputedStyle::default();

        let layout = LayoutBox {
            x,
            y,
            width: self.viewport_width,
            height: 0.0, // Will be calculated
        };

        let children = element
            .children
            .iter()
            .enumerate()
            .map(|(i, child)| self.build_render_node(child, x, y + (i as f32 * 20.0)))
            .collect();

        RenderNode { element: element.clone(), computed_style, layout, children }
    }

    /// Layout the render tree.
    pub fn layout(&mut self, tree: &mut RenderTree) {
        self.layout_node(&mut tree.root, 0.0, 0.0, self.viewport_width);
    }

    /// Layout a single node.
    fn layout_node(&self, node: &mut RenderNode, x: f32, y: f32, available_width: f32) {
        node.layout.x = x;
        node.layout.y = y;
        node.layout.width = available_width;

        let mut child_y = y;
        for child in &mut node.children {
            self.layout_node(child, x, child_y, available_width);
            child_y += child.layout.height + 8.0; // Simple block layout
        }

        node.layout.height = child_y - y;
    }

    /// Update viewport size.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new(1920.0, 1080.0)
    }
}
