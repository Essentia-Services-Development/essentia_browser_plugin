# Essentia Browser Plugin

Pure Rust browser engine with consciousness integration for the Essentia ecosystem.

## Features

- **HTML Parser**: Pure Rust HTML parsing
- **Render Engine**: Integration with essentia_ffui for rendering
- **Consciousness Layer**: Consciousness-aware browsing experience
- **Tab Management**: Multi-tab browsing support

## Usage

```rust
use essentia_browser_plugin::{BrowserPlugin, BrowserConfig};

let plugin = BrowserPlugin::default();
let tab = plugin.new_tab()?;
plugin.navigate(&tab.id, "https://example.com")?;
```

## SSOP Compliance

This plugin is fully SSOP-compliant (std-only, zero third-party dependencies).

## License

MIT
