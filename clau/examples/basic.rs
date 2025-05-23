use clau::{Client, Config};

#[tokio::main]
async fn main() -> clau::Result<()> {
    // Initialize the client with default configuration
    let client = Client::new(Config::default());
    
    // Send a simple query
    let response = client
        .query("What is 2 + 2?")
        .send()
        .await?;
    
    println!("Response: {}", response);
    
    Ok(())
}