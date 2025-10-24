// ZokuZoku for Zed Editor
// Port from VS Code extension

use zed::extension;

pub mod config;
pub mod commands;
pub mod core;
pub mod ipc;
pub mod utils;
pub mod downloader;
pub mod sqlite;
pub mod data_manager;

#[extension]
pub fn init() {
    // Initialize the extension
    tracing::info!("ZokuZoku extension initialized");
}
