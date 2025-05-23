use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_config_path: Option<PathBuf>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    
    #[serde(default)]
    pub stream_format: StreamFormat,
    
    #[serde(default)]
    pub non_interactive: bool,
    
    #[serde(default)]
    pub verbose: bool,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    
    /// Timeout in seconds for Claude CLI execution (default: 30s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamFormat {
    #[default]
    Text,
    Json,
    StreamJson,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            system_prompt: None,
            model: None,
            mcp_config_path: None,
            allowed_tools: None,
            stream_format: StreamFormat::default(),
            non_interactive: true,
            verbose: false,
            max_tokens: None,
            timeout_secs: Some(30), // Default 30 second timeout
        }
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.config.system_prompt = Some(prompt.into());
        self
    }
    
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.config.model = Some(model.into());
        self
    }
    
    
    pub fn mcp_config(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.mcp_config_path = Some(path.into());
        self
    }
    
    pub fn allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.config.allowed_tools = Some(tools);
        self
    }
    
    pub fn stream_format(mut self, format: StreamFormat) -> Self {
        self.config.stream_format = format;
        self
    }
    
    pub fn non_interactive(mut self, non_interactive: bool) -> Self {
        self.config.non_interactive = non_interactive;
        self
    }
    
    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = Some(max_tokens);
        self
    }
    
    pub fn timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.config.timeout_secs = Some(timeout_secs);
        self
    }
    
    pub fn build(self) -> Config {
        self.config
    }
}