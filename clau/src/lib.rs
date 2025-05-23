//! # clau.rs - Rust SDK for Claude Code
//! 
//! A type-safe, async-first Rust SDK for Claude Code that transforms the CLI tool
//! into a powerful programmatic API.
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use clau::{Client, Config};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), clau::Error> {
//!     let client = Client::new(Config::default());
//!     
//!     let response = client
//!         .query("Write a hello world in Rust")
//!         .send()
//!         .await?;
//!     
//!     println!("{}", response);
//!     Ok(())
//! }
//! ```

// Re-export core types
pub use clau_core::{
    Config, Error, Result, Message, MessageType, MessageMeta,
    Session, SessionId, SessionManager,
    StreamFormat, ToolPermission, Cost,
    ClaudeResponse, ResponseMetadata, TokenUsage,
};

// Re-export runtime types
pub use clau_runtime::{
    Client, QueryBuilder, MessageStream,
};

// Re-export MCP types when ready
// pub use clau_mcp::{McpConfig, McpServer};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        Client, Config, Error, Result,
        Message, MessageType, StreamFormat,
    };
    pub use futures::StreamExt;
}