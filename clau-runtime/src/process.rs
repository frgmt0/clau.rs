use clau_core::{Error, Result, Config, StreamFormat};
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use tracing::debug;

/// Execute a one-shot Claude command with timeout
pub async fn execute_claude(config: &Config, query: &str) -> Result<String> {
    
    let claude_binary = which::which("claude").map_err(|_| Error::BinaryNotFound)?;
    
    let mut cmd = Command::new(claude_binary);
    
    // Always use non-interactive mode for SDK
    cmd.arg("-p");
    
    // Add format flag
    match config.stream_format {
        StreamFormat::Json => {
            cmd.arg("--output-format").arg("json");
        }
        StreamFormat::StreamJson => {
            cmd.arg("--output-format").arg("stream-json");
            // stream-json requires verbose flag
            cmd.arg("--verbose");
        }
        StreamFormat::Text => {
            // Text is default, no need to specify
        }
    }
    
    // Add verbose flag if configured (and not already added for stream-json)
    if config.verbose && config.stream_format != StreamFormat::StreamJson {
        cmd.arg("--verbose");
    }
    
    // Add optional flags
    if let Some(system_prompt) = &config.system_prompt {
        cmd.arg("--system-prompt").arg(system_prompt);
    }
    
    if let Some(model) = &config.model {
        cmd.arg("--model").arg(model);
    }
    
    if let Some(mcp_config_path) = &config.mcp_config_path {
        cmd.arg("--mcp-config").arg(mcp_config_path);
    }
    
    if let Some(allowed_tools) = &config.allowed_tools {
        for tool in allowed_tools {
            cmd.arg("--allowedTools").arg(tool);
        }
    }
    
    if let Some(max_tokens) = &config.max_tokens {
        cmd.arg("--max-tokens").arg(max_tokens.to_string());
    }
    
    // Add the query as the last argument
    cmd.arg(query);
    
    debug!("Executing Claude command: {:?}", cmd);
    
    // Execute the command with timeout
    let timeout_duration = Duration::from_secs(config.timeout_secs.unwrap_or(30));
    let output = timeout(timeout_duration, cmd.output())
        .await
        .map_err(|_| Error::Timeout(config.timeout_secs.unwrap_or(30)))??;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ProcessError(format!("Claude command failed: {}", stderr)));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}