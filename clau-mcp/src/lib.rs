//! Model Context Protocol (MCP) implementation for clau.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub servers: Vec<McpServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
}

impl McpServer {
    pub fn new(command: impl Into<String>, args: Vec<&str>) -> Self {
        Self {
            name: String::new(),
            command: command.into(),
            args: args.into_iter().map(String::from).collect(),
            env: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }
}

// TODO: Implement full MCP protocol support