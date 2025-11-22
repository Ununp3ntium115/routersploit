// MCP Server Implementation

use anyhow::{Result, anyhow};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io::Write;

use super::schema::*;
use super::handlers::MCPHandlers;

pub async fn start(transport: &str) -> Result<()> {
    match transport {
        "stdio" => start_stdio_server().await,
        "http" => start_http_server().await,
        _ => Err(anyhow!("Unsupported transport: {}", transport)),
    }
}

async fn start_stdio_server() -> Result<()> {
    tracing::info!("MCP server starting on stdio");

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut lines = BufReader::new(stdin).lines();

    // Send initialization message
    send_response(&mut stdout, None, Some(json!({
        "server": "pyroutersploit-mcp",
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": {
            "tools": true,
            "resources": false,
            "prompts": false
        }
    }))).await?;

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<MCPRequest>(&line) {
            Ok(request) => {
                let response = handle_request(request).await;
                send_response_obj(&mut stdout, response).await?;
            }
            Err(e) => {
                tracing::error!("Failed to parse request: {}", e);
                let error_response = MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(MCPError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                };
                send_response_obj(&mut stdout, error_response).await?;
            }
        }
    }

    Ok(())
}

async fn start_http_server() -> Result<()> {
    tracing::info!("MCP HTTP server not yet implemented");
    Err(anyhow!("HTTP transport not yet implemented"))
}

async fn handle_request(request: MCPRequest) -> MCPResponse {
    tracing::debug!("Handling request: method={}", request.method);

    let result = match request.method.as_str() {
        "tools/list" => MCPHandlers::list_tools(),

        "tools/call" => {
            let params = request.params.ok_or_else(|| anyhow!("Missing params"))?;
            let tool_name = params.get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing tool name"))?;
            let arguments = params.get("arguments")
                .ok_or_else(|| anyhow!("Missing arguments"))?;

            handle_tool_call(tool_name, arguments.clone())
        }

        "initialize" => {
            Ok(json!({
                "server": "pyroutersploit-mcp",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": {
                    "tools": true
                }
            }))
        }

        _ => Err(anyhow!("Unknown method: {}", request.method)),
    };

    match result {
        Ok(value) => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(value),
            error: None,
        },
        Err(e) => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(MCPError {
                code: -32603,
                message: e.to_string(),
                data: None,
            }),
        },
    }
}

fn handle_tool_call(tool_name: &str, arguments: Value) -> Result<Value> {
    match tool_name {
        "cryptex_query" => {
            let params: CryptexQueryParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_cryptex_query(params)
        }
        "cryptex_add" => {
            let params: CryptexAddParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_cryptex_add(params)
        }
        "list_exploits" => {
            MCPHandlers::handle_list_exploits()
        }
        "run_exploit" => {
            let params: ExploitRunParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_run_exploit(params)
        }
        "scan_target" => {
            let params: ScanParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_scan_target(params)
        }
        "multi_hash" => {
            let params: HashParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_multi_hash(params)
        }
        "qkd_encrypt" => {
            let params: QKDEncryptParams = serde_json::from_value(arguments)?;
            MCPHandlers::handle_qkd_encrypt(params)
        }
        _ => Err(anyhow!("Unknown tool: {}", tool_name)),
    }
}

async fn send_response(
    stdout: &mut tokio::io::Stdout,
    id: Option<Value>,
    result: Option<Value>,
) -> Result<()> {
    let response = MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result,
        error: None,
    };
    send_response_obj(stdout, response).await
}

async fn send_response_obj(
    stdout: &mut tokio::io::Stdout,
    response: MCPResponse,
) -> Result<()> {
    let json = serde_json::to_string(&response)?;
    stdout.write_all(json.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_list_tools() {
        crate::db::redb_client::init_database().await.unwrap();
        let result = MCPHandlers::list_tools().unwrap();
        assert!(result.get("tools").is_some());
    }
}
