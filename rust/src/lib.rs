// PyRouterSploit - Cross-platform Security Exploitation Framework
// Rust Core Library

pub mod core;
pub mod crypto;
pub mod db;
pub mod api;
pub mod mcp;
pub mod nodered;
pub mod python_compat;

// Re-exports
pub use crate::core::{exploit, scanner, payload, session};
pub use crate::crypto::{qkd, hashing, pqc};
pub use crate::db::{redb_client, cryptex, models};
pub use crate::api::{rest, websocket};
pub use crate::mcp::server as mcp_server;

use anyhow::Result;
use tracing::{info, error};

/// Initialize the PyRouterSploit library
pub async fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("PyRouterSploit v{} initializing...", env!("CARGO_PKG_VERSION"));

    // Initialize database
    db::redb_client::init_database().await?;

    // Initialize crypto subsystem
    crypto::init()?;

    info!("PyRouterSploit initialized successfully");
    Ok(())
}

/// Shutdown and cleanup
pub async fn shutdown() -> Result<()> {
    info!("PyRouterSploit shutting down...");
    db::redb_client::close_database().await?;
    Ok(())
}
