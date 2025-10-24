# ZokuZoku for Zed Editor

This is a port of the ZokuZoku VS Code extension to Zed Editor, written in Rust.

## Status

🔄 **In Progress** - This is an experimental rewrite

## Architecture

### Key Differences from VS Code Version

1. **Language**: TypeScript → Rust
2. **Runtime**: Node.js → Rust (compiled to WebAssembly)
3. **Python Integration**: Uses PyO3 instead of pymport
4. **UI Framework**: VS Code Webviews → Zed UI (to be implemented)
5. **HTTP Client**: Node.js `http` → Rust `reqwest`

## Components

### Completed ✅
- Basic project structure
- Configuration management (`config.rs`)
- IPC communication (`ipc.rs`)
- Utility functions (`utils.rs`)
- Command handler scaffolding (`commands.rs`)

### In Progress 🔄
- File operations
- SQLite integration
- Python bridge

### Pending 📋
- Unity asset parsing
- UI components
- Tree views
- Custom editors

## Building

```bash
cd ZedTest
cargo build --release
```

## Configuration

Configuration is stored in `~/.config/zokuzoku/config.json` (Linux) or `%APPDATA%\zokuzoku\config.json` (Windows).

## Migration Notes

### Python Integration
The VS Code version uses `pymport` to run Python code. For Zed, we plan to use `PyO3` which provides Rust bindings for Python. This allows calling Python code directly from Rust without spawning subprocesses.

### Unity Asset Parsing
The VS Code version uses UnityPy (Python library). For Zed, we need to either:
1. Use PyO3 to call UnityPy from Rust
2. Write a native Rust implementation
3. Use a hybrid approach

## Development

This is a work in progress. See TODO items in the code for planned features.

## License

GPL-3.0 (same as original ZokuZoku)

