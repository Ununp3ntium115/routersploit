// MCP Protocol Schema and Types

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// MCP Tool Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Cryptex query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptexQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// Cryptex add entry parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptexAddParams {
    pub function_name: String,
    pub branding_name: String,
    pub pseudo_code: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_impl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python_impl: Option<String>,
}

/// Exploit execution parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploitRunParams {
    pub exploit_id: Option<String>,
    pub branding_name: Option<String>,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// Scan parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanParams {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<usize>,
}

/// Hash parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashParams {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(default)]
    pub all_algorithms: bool,
}

/// QKD encryption parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QKDEncryptParams {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<usize>,
}
