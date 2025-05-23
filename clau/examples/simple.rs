use clau::{Client, StreamFormat};

#[tokio::main]
async fn main() -> clau::Result<()> {
    // Create client with JSON format
    let client = Client::builder()
        .stream_format(StreamFormat::Json)
        .build();
    
    println!("Sending query...");
    match client.query("Say just hello").send().await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    
    Ok(())
}