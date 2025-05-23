use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Claude Code not found in PATH")]
    BinaryNotFound,
    
    #[error("Session {0} not found")]
    SessionNotFound(String),
    
    #[error("Tool permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("MCP server error: {0}")]
    McpError(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Operation timed out after {0}s")]
    Timeout(u64),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Process error: {0}")]
    ProcessError(String),
    
    #[error("Stream closed unexpectedly")]
    StreamClosed,
}

pub type Result<T> = std::result::Result<T, Error>;