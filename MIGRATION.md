# ZokuZoku Zed Migration Plan

## Overview

This document outlines the migration plan from the VS Code extension to Zed Editor.

## Phase 1: Foundation ✅ (Completed)
- [x] Project structure
- [x] Configuration management
- [x] IPC communication
- [x] Basic utilities

## Phase 2: Core Operations (In Progress)
- [ ] Downloader implementation
- [ ] SQLite integration with encryption support
- [ ] File system operations
- [ ] JSON document handling

## Phase 3: Python Integration
- [ ] PyO3 setup
- [ ] Python bridge implementation
- [ ] UnityPy integration
- [ ] APSW integration for encrypted SQLite

## Phase 4: Unity Asset Parsing
- [ ] Asset bundle parsing
- [ ] Text asset extraction
- [ ] Story data extraction
- [ ] Race story data extraction
- [ ] Lyrics data extraction

## Phase 5: UI Components
- [ ] Tree view implementation
- [ ] Custom editor framework
- [ ] Story editor UI
- [ ] Lyrics editor UI
- [ ] MDB editor UI
- [ ] Localize dict editor UI

## Phase 6: Integration
- [ ] Command registration
- [ ] Workspace integration
- [ ] Settings UI
- [ ] Error handling
- [ ] Logging

## Phase 7: Testing & Polish
- [ ] Unit tests
- [ ] Integration tests
- [ ] Performance optimization
- [ ] Documentation

## Key Technical Decisions

### 1. Python Integration
**Decision**: Use PyO3 for direct Python integration
**Rationale**: Allows calling Python code directly from Rust without subprocess overhead

### 2. UI Framework
**Decision**: Use Zed's native UI system
**Rationale**: Need to wait for Zed's UI API to mature

### 3. SQLite Encryption
**Decision**: Use rusqlite with custom encryption support
**Rationale**: Need to maintain compatibility with encrypted `meta` database

### 4. Configuration Storage
**Decision**: Use config crate with JSON storage
**Rationale**: Simple, cross-platform configuration management

## Dependencies

### Core
- `zed` - Zed extension API
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `rusqlite` - SQLite database
- `serde` - Serialization
- `anyhow` - Error handling

### Python
- `pyo3` - Python bindings
- `unitypy` (Python) - Unity asset parsing
- `apsw-sqlite3mc` (Python) - Encrypted SQLite

### Utilities
- `tar` - Archive extraction
- `walkdir` - Directory traversal
- `config` - Configuration management

## File Structure

```
ZedTest/
├── Cargo.toml
├── README.md
├── MIGRATION.md (this file)
└── src/
    ├── lib.rs              # Main entry point
    ├── main.rs             # CLI entry point
    ├── config.rs           # Configuration management
    ├── commands.rs         # Command handlers
    ├── core.rs             # Core exports
    ├── ipc.rs              # Hachimi IPC communication
    ├── utils.rs            # Utility functions
    ├── downloader.rs       # HTTP downloader
    └── sqlite.rs           # SQLite management
```

## Challenges

### 1. Python Integration Complexity
**Challenge**: VS Code uses pymport (Node.js Python binding)
**Solution**: Use PyO3 for direct Python integration

### 2. UI Framework
**Challenge**: Zed UI API is still evolving
**Solution**: Start with CLI, add UI later as API matures

### 3. Asset Parsing
**Challenge**: UnityPy is Python-only
**Solution**: Use PyO3 to call UnityPy from Rust

### 4. File System Operations
**Challenge**: Complex path handling across platforms
**Solution**: Use Rust's Path/PathBuf with proper error handling

## Success Criteria

1. ✅ All core functionality ported
2. ✅ Python integration working
3. ✅ Unity asset parsing functional
4. ✅ UI components implemented
5. ✅ Performance comparable to VS Code version
6. ✅ Maintainable codebase

## Timeline Estimate

- Phase 1: ✅ Complete
- Phase 2: 2 weeks
- Phase 3: 3 weeks
- Phase 4: 4 weeks
- Phase 5: 6 weeks (depending on Zed UI API)
- Phase 6: 2 weeks
- Phase 7: 2 weeks

**Total**: ~19 weeks (~4.5 months)

## Notes

- This is an experimental port
- Zed's extension API may change significantly
- Some features may need to be redesigned for Zed's architecture
- Keep original VS Code version maintained in parallel

