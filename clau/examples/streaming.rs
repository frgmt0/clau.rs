use clau::prelude::*;

#[tokio::main]
async fn main() -> clau::Result<()> {
    // Create a client with streaming JSON format
    let client = Client::builder()
        .model("claude-sonnet-4-20250514")
        .stream_format(StreamFormat::StreamJson)
        .build();

    // Create a streaming query
    let mut stream = client
        .query("Explain the concept of ownership in Rust in 3 paragraphs")
        .stream()
        .await?;

    println!("Streaming response:");
    println!("==================");

    // Process messages as they arrive
    while let Some(message) = stream.next().await {
        match message? {
            Message::Init { meta } => {
                println!("Starting conversation (session: {})", meta.session_id);
            }
            Message::Assistant { content, .. } => {
                print!("{}", content);
                // Flush to see output immediately
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
            }
            Message::Result { meta: _, stats } => {
                println!("\n\nConversation complete!");
                println!("Total cost: ${:.4}", stats.total_cost_usd);
                println!("Duration: {}ms", stats.total_duration_ms);
            }
            _ => {}
        }
    }

    Ok(())
}
