use clau_core::{Error, Result, Message, StreamFormat};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tracing::error;

pub struct MessageStream {
    receiver: mpsc::Receiver<Result<Message>>,
}

impl MessageStream {
    pub fn new(receiver: mpsc::Receiver<Result<Message>>, _format: StreamFormat) -> Self {
        Self { receiver }
    }
    
    pub async fn collect_full_response(mut self) -> Result<String> {
        let mut response = String::new();
        
        while let Some(result) = self.next().await {
            match result? {
                Message::Assistant { content, .. } => {
                    response.push_str(&content);
                }
                Message::Result { .. } => {
                    // End of conversation
                    break;
                }
                _ => {}
            }
        }
        
        Ok(response)
    }
}

impl Stream for MessageStream {
    type Item = Result<Message>;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

pub struct MessageParser {
    format: StreamFormat,
}

impl MessageParser {
    pub fn new(format: StreamFormat) -> Self {
        Self { format }
    }
    
    pub fn parse_line(&self, line: &str) -> Result<Option<Message>> {
        match self.format {
            StreamFormat::Text => {
                // Text format doesn't have structured messages
                Ok(None)
            }
            StreamFormat::Json | StreamFormat::StreamJson => {
                if line.trim().is_empty() {
                    return Ok(None);
                }
                
                match serde_json::from_str::<Message>(line) {
                    Ok(message) => Ok(Some(message)),
                    Err(e) => {
                        error!("Failed to parse message: {}, line: {}", e, line);
                        Err(Error::SerializationError(e))
                    }
                }
            }
        }
    }
    
    pub fn parse_text_response(&self, text: &str) -> Message {
        // For text format, create a simple assistant message
        Message::Assistant {
            content: text.to_string(),
            meta: clau_core::MessageMeta {
                session_id: "text-response".to_string(),
                timestamp: Some(std::time::SystemTime::now()),
                cost_usd: None,
                duration_ms: None,
                tokens_used: None,
            },
        }
    }
}