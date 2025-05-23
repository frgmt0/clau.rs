# clau.rs 🦀

A type-safe, async-first Rust SDK for [Claude Code](https://github.com/anthropics/claude-code) that transforms the CLI tool into a powerful programmatic API.

## Features

- **Type-Safe API**: Strongly typed request/response models with compile-time safety
- **Async/Await**: Built on Tokio for efficient async operations
- **Raw JSON Access**: Full access to Claude CLI responses for maximum flexibility
- **Timeout Support**: Configurable timeouts for all operations
- **Streaming Support**: Real-time streaming responses with async iterators
- **Session Management**: First-class session support with persistence
- **MCP Integration**: Type-safe Model Context Protocol configuration
- **Error Handling**: Comprehensive error types with detailed messages

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clau = "0.1.0"
```

## Quick Start

```rust
use clau::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), clau::Error> {
    let client = Client::new(Config::default());

    let response = client
        .query("Write a hello world in Rust")
        .send()
        .await?;

    println!("{}", response);
    Ok(())
}
```

## Advanced Features

### Raw JSON Access

Access the complete response with metadata and raw JSON for maximum flexibility:

```rust
use clau::{Client, StreamFormat};

let client = Client::builder()
    .stream_format(StreamFormat::Json)
    .build();

let response = client
    .query("What is Rust?")
    .send_full()
    .await?;

println!("Content: {}", response.content);

// Access metadata
if let Some(metadata) = &response.metadata {
    println!("Cost: ${:.6}", metadata.cost_usd.unwrap_or(0.0));
    println!("Session: {}", metadata.session_id);
}

// Access raw JSON for custom parsing
if let Some(raw_json) = &response.raw_json {
    // Extract any field from the Claude CLI response
    let custom_data = raw_json.get("duration_api_ms");
}

// Serialize the entire response for storage
let json_string = serde_json::to_string(&response)?;
```

### Timeout Configuration

```rust
let client = Client::builder()
    .timeout_secs(60) // 60 second timeout
    .build();
```

## Examples

### Streaming Responses

```rust
use clau::prelude::*;

let mut stream = client
    .query("Explain quantum computing")
    .format(StreamFormat::StreamingJson)
    .stream()
    .await?;

while let Some(message) = stream.next().await {
    match message? {
        Message::Assistant { content, .. } => print!("{}", content),
        Message::Result { stats, .. } => println!("\nCost: ${}", stats.total_cost_usd),
        _ => {}
    }
}
```

### Custom Configuration

```rust
use clau::{Client, Config, StreamFormat};

let client = Client::builder()
    .system_prompt("You are a Rust expert")
    .model("claude-sonnet-4-20250514")
    .allowed_tools(vec!["filesystem".to_string()])
    .stream_format(StreamFormat::Json)
    .build();
```

### Session Management

```rust
let session_manager = SessionManager::new();

let session = session_manager
    .create_session()
    .with_system_prompt("You are a helpful assistant")
    .build()
    .await?;

// Continue conversation with session ID
let response = client
    .query("What did we discuss earlier?")
    .session(session.id().clone())
    .send()
    .await?;
```

## Architecture

The SDK is organized into modular crates:

- `clau-core`: Core types and traits
- `clau-runtime`: Process management and I/O
- `clau-mcp`: MCP protocol implementation
- `clau-macros`: Derive macros for custom types

## Requirements

- Rust 1.70+
- Claude Code CLI installed and authenticated

## License

MIT

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Links

- [Documentation](https://docs.rs/clau) <- Not Up Yet
- [Repository](https://github.com/frgmt0/clau.rs)
- [Claude Code](https://github.com/anthropics/claude-code)
