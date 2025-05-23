use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Init,
    User,
    Assistant,
    Result,
    System,
    Tool,
    ToolResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMeta {
    pub session_id: String,
    pub timestamp: Option<SystemTime>,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub tokens_used: Option<TokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input: u64,
    pub output: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Init {
        #[serde(flatten)]
        meta: MessageMeta,
    },
    User {
        content: String,
        #[serde(flatten)]
        meta: MessageMeta,
    },
    Assistant {
        content: String,
        #[serde(flatten)]
        meta: MessageMeta,
    },
    Result {
        #[serde(flatten)]
        meta: MessageMeta,
        stats: ConversationStats,
    },
    System {
        content: String,
        #[serde(flatten)]
        meta: MessageMeta,
    },
    Tool {
        name: String,
        parameters: serde_json::Value,
        #[serde(flatten)]
        meta: MessageMeta,
    },
    ToolResult {
        tool_name: String,
        result: serde_json::Value,
        #[serde(flatten)]
        meta: MessageMeta,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStats {
    pub total_messages: u64,
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_tokens: TokenUsage,
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            Message::Init { .. } => MessageType::Init,
            Message::User { .. } => MessageType::User,
            Message::Assistant { .. } => MessageType::Assistant,
            Message::Result { .. } => MessageType::Result,
            Message::System { .. } => MessageType::System,
            Message::Tool { .. } => MessageType::Tool,
            Message::ToolResult { .. } => MessageType::ToolResult,
        }
    }
    
    pub fn meta(&self) -> &MessageMeta {
        match self {
            Message::Init { meta, .. } |
            Message::User { meta, .. } |
            Message::Assistant { meta, .. } |
            Message::Result { meta, .. } |
            Message::System { meta, .. } |
            Message::Tool { meta, .. } |
            Message::ToolResult { meta, .. } => meta,
        }
    }
}