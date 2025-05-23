use serde::{Deserialize, Serialize};

/// Response from claude CLI in JSON format (legacy single response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCliResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub subtype: String,
    pub cost_usd: f64,
    pub is_error: bool,
    pub duration_ms: u64,
    pub duration_api_ms: Option<u64>,
    pub num_turns: u32,
    pub result: String,
    pub total_cost: f64,
    pub session_id: String,
}

/// Comprehensive response that includes both parsed content and raw data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResponse {
    /// The main text content from Claude
    pub content: String,
    
    /// Raw JSON response from Claude CLI for advanced parsing
    pub raw_json: Option<serde_json::Value>,
    
    /// Structured metadata when available
    pub metadata: Option<ResponseMetadata>,
}

/// Structured metadata extracted from Claude CLI responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub session_id: String,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub tokens_used: Option<TokenUsage>,
    pub model: Option<String>,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

impl ClaudeResponse {
    /// Create a simple text response
    pub fn text(content: String) -> Self {
        Self {
            content,
            raw_json: None,
            metadata: None,
        }
    }
    
    /// Create a response with full JSON data
    pub fn with_json(content: String, raw_json: serde_json::Value) -> Self {
        let metadata = Self::extract_metadata(&raw_json);
        Self {
            content,
            raw_json: Some(raw_json),
            metadata,
        }
    }
    
    /// Extract metadata from raw JSON response
    fn extract_metadata(json: &serde_json::Value) -> Option<ResponseMetadata> {
        let session_id = json.get("session_id")?.as_str()?.to_string();
        
        Some(ResponseMetadata {
            session_id,
            cost_usd: json.get("cost_usd").and_then(|v| v.as_f64()),
            duration_ms: json.get("duration_ms").and_then(|v| v.as_u64()),
            tokens_used: json.get("message")
                .and_then(|m| m.get("usage"))
                .map(|usage| TokenUsage {
                    input_tokens: usage.get("input_tokens").and_then(|v| v.as_u64()),
                    output_tokens: usage.get("output_tokens").and_then(|v| v.as_u64()),
                    cache_creation_input_tokens: usage.get("cache_creation_input_tokens").and_then(|v| v.as_u64()),
                    cache_read_input_tokens: usage.get("cache_read_input_tokens").and_then(|v| v.as_u64()),
                }),
            model: json.get("message")
                .and_then(|m| m.get("model"))
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolPermission {
    Mcp {
        server: String,
        tool: String,
    },
    Bash {
        command: String,
    },
    All,
}

impl ToolPermission {
    pub fn mcp(server: impl Into<String>, tool: impl Into<String>) -> Self {
        Self::Mcp {
            server: server.into(),
            tool: tool.into(),
        }
    }
    
    pub fn bash(command: impl Into<String>) -> Self {
        Self::Bash {
            command: command.into(),
        }
    }
    
    pub fn to_cli_format(&self) -> String {
        match self {
            Self::Mcp { server, tool } => {
                if tool == "*" {
                    format!("mcp__{}__*", server)
                } else {
                    format!("mcp__{}__{}", server, tool)
                }
            }
            Self::Bash { command } => format!("bash:{}", command),
            Self::All => "*".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cost {
    pub usd: f64,
}

impl Cost {
    pub fn new(usd: f64) -> Self {
        Self { usd }
    }
    
    pub fn zero() -> Self {
        Self { usd: 0.0 }
    }
    
    pub fn add(&self, other: &Self) -> Self {
        Self {
            usd: self.usd + other.usd,
        }
    }
}