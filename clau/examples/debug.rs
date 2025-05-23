use clau::{Client, Config};

#[tokio::main]
async fn main() -> clau::Result<()> {
    let client = Client::new(Config::default());
    
    println!("Sending query...");
    match client.query("Hi").send().await {
        Ok(response) => {
            println!("Got response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    
    Ok(())
}