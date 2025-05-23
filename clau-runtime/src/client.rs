use clau_core::{Config, Message, Result, SessionId, StreamFormat, ClaudeCliResponse, ClaudeResponse};
use crate::{MessageStream, process::execute_claude};
use std::sync::Arc;
use tokio::sync::mpsc;

/// High-level client for interacting with Claude Code CLI
/// 
/// The `Client` provides a type-safe, async interface to Claude Code with support
/// for different output formats, configuration options, and both simple and advanced
/// response handling.
/// 
/// # Examples
/// 
/// Basic usage:
/// ```rust,no_run
/// # use clau_core::*;
/// # #[tokio::main]
/// # async fn main() -> clau_core::Result<()> {
/// let client = Client::new(Config::default());
/// let response = client.query("Hello").send().await?;
/// println!("{}", response);
/// # Ok(())
/// # }
/// ```
/// 
/// With configuration:
/// ```rust,no_run
/// # use clau_core::*;
/// # #[tokio::main]
/// # async fn main() -> clau_core::Result<()> {
/// let client = Client::builder()
///     .model("claude-3-opus-20240229")
///     .stream_format(StreamFormat::Json)
///     .timeout_secs(60)
///     .build();
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Client {
    config: Arc<Config>,
}

impl Client {
    /// Create a new client with the given configuration
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
    
    /// Create a new client builder for fluent configuration
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
    
    /// Create a query builder for the given query string
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// # use clau_core::*;
    /// # #[tokio::main]
    /// # async fn main() -> clau_core::Result<()> {
    /// let client = Client::new(Config::default());
    /// let response = client
    ///     .query("Explain Rust ownership")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn query(&self, query: impl Into<String>) -> QueryBuilder {
        QueryBuilder::new(self.clone(), query.into())
    }
    
    /// Send a query and return just the text content (backwards compatible)
    /// 
    /// This is the simplest way to get a response from Claude. For access to 
    /// metadata, costs, and raw JSON, use [`send_full`](Self::send_full).
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// # use clau_core::*;
    /// # #[tokio::main]
    /// # async fn main() -> clau_core::Result<()> {
    /// let client = Client::new(Config::default());
    /// let answer = client.send("What is 2 + 2?").await?;
    /// assert_eq!(answer.trim(), "4");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, query: &str) -> Result<String> {
        let response = self.send_full(query).await?;
        Ok(response.content)
    }
    
    /// Send a query and return the full response with metadata and raw JSON
    /// 
    /// This method provides access to the complete response from Claude Code,
    /// including metadata like costs, session IDs, and the raw JSON for 
    /// advanced parsing or storage.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// # use clau_core::*;
    /// # #[tokio::main]
    /// # async fn main() -> clau_core::Result<()> {
    /// let client = Client::builder()
    ///     .stream_format(StreamFormat::Json)
    ///     .build();
    /// 
    /// let response = client.send_full("Hello").await?;
    /// println!("Content: {}", response.content);
    /// 
    /// if let Some(metadata) = &response.metadata {
    ///     println!("Cost: ${:.6}", metadata.cost_usd.unwrap_or(0.0));
    ///     println!("Session: {}", metadata.session_id);
    /// }
    /// 
    /// // Access raw JSON for custom parsing
    /// if let Some(raw) = &response.raw_json {
    ///     // Custom field extraction
    ///     let custom_field = raw.get("custom_field");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_full(&self, query: &str) -> Result<ClaudeResponse> {
        let output = execute_claude(&self.config, query).await?;
        
        // Parse response based on format
        match self.config.stream_format {
            StreamFormat::Text => {
                Ok(ClaudeResponse::text(output.trim().to_string()))
            }
            StreamFormat::Json => {
                // Parse the JSON response from claude CLI
                let json_value: serde_json::Value = serde_json::from_str(&output)?;
                let claude_response: ClaudeCliResponse = serde_json::from_value(json_value.clone())?;
                Ok(ClaudeResponse::with_json(claude_response.result, json_value))
            }
            StreamFormat::StreamJson => {
                // For stream-json, we need to parse multiple JSON lines
                let mut result = String::new();
                let mut all_json = Vec::new();
                
                for line in output.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    // Try to parse as a message
                    if let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) {
                        all_json.push(msg.clone());
                        
                        // Check if it's an assistant message
                        if msg.get("type").and_then(|v| v.as_str()) == Some("assistant") {
                            // Extract text from the message content
                            if let Some(message) = msg.get("message") {
                                if let Some(content_array) = message.get("content").and_then(|v| v.as_array()) {
                                    for content_item in content_array {
                                        if content_item.get("type").and_then(|v| v.as_str()) == Some("text") {
                                            if let Some(text) = content_item.get("text").and_then(|v| v.as_str()) {
                                                result.push_str(text);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Return the response with all JSON messages as an array
                let raw_json = serde_json::Value::Array(all_json);
                Ok(ClaudeResponse::with_json(result, raw_json))
            }
        }
    }
}

pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
    
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.config.system_prompt = Some(prompt.into());
        self
    }
    
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.config.model = Some(model.into());
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
    
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }
    
    pub fn timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.config.timeout_secs = Some(timeout_secs);
        self
    }
    
    pub fn build(self) -> Client {
        Client::new(self.config)
    }
}

pub struct QueryBuilder {
    client: Client,
    query: String,
    session_id: Option<SessionId>,
    format: Option<StreamFormat>,
}

impl QueryBuilder {
    fn new(client: Client, query: String) -> Self {
        Self {
            client,
            query,
            session_id: None,
            format: None,
        }
    }
    
    pub fn session(mut self, session_id: SessionId) -> Self {
        self.session_id = Some(session_id);
        self
    }
    
    pub fn format(mut self, format: StreamFormat) -> Self {
        self.format = Some(format);
        self
    }
    
    /// Send the query and return just the text content
    pub async fn send(self) -> Result<String> {
        self.client.send(&self.query).await
    }
    
    /// Send the query and return the full response with metadata and raw JSON
    pub async fn send_full(self) -> Result<ClaudeResponse> {
        self.client.send_full(&self.query).await
    }
    
    pub async fn stream(self) -> Result<MessageStream> {
        // For now, streaming is simulated by getting the full response
        // and sending it as a single message
        let (tx, rx) = mpsc::channel(100);
        
        let format = self.format.unwrap_or(self.client.config.stream_format);
        let client = self.client;
        let query = self.query;
        
        tokio::spawn(async move {
            match client.send(&query).await {
                Ok(response) => {
                    // Send the response as a single assistant message
                    let msg = Message::Assistant {
                        content: response,
                        meta: clau_core::MessageMeta {
                            session_id: "stream-session".to_string(),
                            timestamp: Some(std::time::SystemTime::now()),
                            cost_usd: None,
                            duration_ms: None,
                            tokens_used: None,
                        },
                    };
                    let _ = tx.send(Ok(msg)).await;
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                }
            }
        });
        
        Ok(MessageStream::new(rx, format))
    }
    
    pub async fn parse_output<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let response = self.send().await?;
        serde_json::from_str(&response).map_err(Into::into)
    }
}

