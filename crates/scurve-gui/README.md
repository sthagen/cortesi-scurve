# Scurve GUI

Interactive space-filling curve visualization GUI library and web application.

This crate provides the GUI components for space-filling curve visualization. It can be used as a library by other applications (like the `scurve` CLI) or run directly as a web application.

## Features

- **Complete GUI functionality**: Interactive 2D and 3D curve visualizations
- **Web-first design**: Optimized for WebGL2 rendering in browsers
- **Library interface**: Can be imported by other Rust applications
- **Multiple curve types**: Support for various space-filling curve algorithms
- **Responsive design**: Adapts to different screen sizes

## Usage

### As a Library
Add to your `Cargo.toml`:
```toml
[dependencies]
scurve-gui = { path = "../scurve-gui" }
```

Then call the GUI function:
```rust
scurve_gui::gui()?;
```

### Web Application
1. **Install required tools (run from the repository root):**
   ```bash
   cargo xtask web setup
   ```

2. **Run development server (run from the repository root):**
   ```bash
   cargo xtask web serve
   ```

   Open `http://127.0.0.1:1334` in your browser (uses wasm-server-runner).

## Dependencies

### Core Dependencies
- **spacecurve** - Space-filling curve generation algorithms
- **anyhow** - Error handling
- **egui / eframe** - GUI framework
- **getrandom 0.3** - Random number generation with WASM support
- **egui_commonmark** - Markdown rendering for egui

### Web-specific Dependencies
- **wasm-bindgen** - Rust/JavaScript interop
- **console_error_panic_hook** - Better error reporting in browsers

## Project Structure

```
crates/scurve-gui/
├── src/
│   ├── lib.rs           # Library exports
│   └── web.rs           # Web-only binary (GUI only)
├── assets/              # Web assets
│   └── index.html       # Web page template
├── index.html           # Symlink to assets/index.html
├── tests/               # Integration tests (including wasm build coverage)
│   └── web_build.rs
├── README.md            # This file
```

## Build Targets

The crate supports multiple build targets:

- **Native executable**: `scurve` - Full CLI with GUI support
- **Web application**: `scurve-web` - GUI-only for browser deployment
- **Library**: Available as both cdylib and rlib for integration

## Requirements

### Native
- Rust toolchain
- Graphics drivers supporting OpenGL

### Web
- Rust toolchain with `wasm32-unknown-unknown` target
- wasm-server-runner for development (installed by `cargo xtask web setup`)
- Modern web browser with WebGL2 support

## Browser Compatibility

Tested and working on:
- Chrome 80+
- Firefox 79+
- Safari 14+
- Edge 80+

Requires WebGL2 support (available in all modern browsers).
