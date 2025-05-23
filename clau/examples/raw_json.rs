use clau::{Client, StreamFormat, ClaudeResponse};

#[tokio::main]
async fn main() -> clau::Result<()> {
    // Create client with JSON format to get full metadata
    let client = Client::builder()
        .stream_format(StreamFormat::Json)
        .build();
    
    println!("Sending query with full response...");
    
    // Get the full response with metadata and raw JSON
    let response: ClaudeResponse = client
        .query("What is 2 + 2? Keep it brief.")
        .send_full()
        .await?;
    
    println!("Content: {}", response.content);
    println!("\n--- Metadata ---");
    if let Some(metadata) = &response.metadata {
        println!("Session ID: {}", metadata.session_id);
        if let Some(cost) = metadata.cost_usd {
            println!("Cost: ${:.6}", cost);
        }
        if let Some(duration) = metadata.duration_ms {
            println!("Duration: {}ms", duration);
        }
        if let Some(model) = &metadata.model {
            println!("Model: {}", model);
        }
        if let Some(tokens) = &metadata.tokens_used {
            if let Some(input) = tokens.input_tokens {
                println!("Input tokens: {}", input);
            }
            if let Some(output) = tokens.output_tokens {
                println!("Output tokens: {}", output);
            }
        }
    }
    
    println!("\n--- Raw JSON ---");
    if let Some(raw_json) = &response.raw_json {
        // Pretty print the raw JSON
        println!("{}", serde_json::to_string_pretty(raw_json)?);
        
        // Example: Extract custom fields
        if let Some(total_cost) = raw_json.get("total_cost").and_then(|v| v.as_f64()) {
            println!("\nTotal cost from raw JSON: ${:.6}", total_cost);
        }
    }
    
    println!("\n--- Serialized Response ---");
    // The entire response can be serialized for storage/transmission
    let serialized = serde_json::to_string_pretty(&response)?;
    println!("{}", serialized);
    
    Ok(())
}