use clau::{Client, ToolPermission};

#[tokio::main]
async fn main() -> clau::Result<()> {
    // Create a client with tool permissions
    let client = Client::builder()
        .system_prompt("You are a helpful coding assistant")
        .allowed_tools(vec![
            ToolPermission::mcp("filesystem", "*").to_cli_format(),
            ToolPermission::bash("npm install").to_cli_format(),
            ToolPermission::bash("npm run").to_cli_format(),
        ])
        .build();
    
    // Query with tools enabled
    let response = client
        .query("Can you check what files are in the current directory?")
        .send()
        .await?;
    
    println!("Response: {}", response);
    
    Ok(())
}