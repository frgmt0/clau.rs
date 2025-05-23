use clau::{Config, StreamFormat};

#[test]
fn test_config_builder() {
    let config = Config::builder()
        .system_prompt("You are a helpful assistant")
        .model("claude-sonnet-4-20250514")
        .stream_format(StreamFormat::Json)
        .max_tokens(1000)
        .build();

    assert_eq!(
        config.system_prompt,
        Some("You are a helpful assistant".to_string())
    );
    assert_eq!(config.model, Some("claude-sonnet-4-20250514".to_string()));
    assert_eq!(config.stream_format, StreamFormat::Json);
    assert_eq!(config.max_tokens, Some(1000));
}

#[test]
fn test_session_id() {
    use clau::SessionId;

    let id1 = SessionId::new("test-session");
    let id2 = SessionId::new("test-session");

    assert_eq!(id1, id2);
    assert_eq!(id1.as_str(), "test-session");
}

#[test]
fn test_tool_permission_formatting() {
    use clau::ToolPermission;

    let mcp_all = ToolPermission::mcp("filesystem", "*");
    assert_eq!(mcp_all.to_cli_format(), "mcp__filesystem__*");

    let mcp_specific = ToolPermission::mcp("filesystem", "read_file");
    assert_eq!(mcp_specific.to_cli_format(), "mcp__filesystem__read_file");

    let bash_cmd = ToolPermission::bash("npm install");
    assert_eq!(bash_cmd.to_cli_format(), "bash:npm install");

    let all = ToolPermission::All;
    assert_eq!(all.to_cli_format(), "*");
}
