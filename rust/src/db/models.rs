// Data models for PyRouterSploit

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Cryptex Dictionary Entry
/// Maps function names to branding names and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptexEntry {
    pub id: Uuid,
    pub function_name: String,
    pub branding_name: String,
    pub pseudo_code: String,
    pub rust_impl: Option<String>,
    pub python_impl: Option<String>,
    pub category: CryptexCategory,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CryptexCategory {
    Exploit,
    Scanner,
    Credential,
    Payload,
    Encoder,
    Utility,
}

impl CryptexEntry {
    pub fn new(
        function_name: String,
        branding_name: String,
        pseudo_code: String,
        category: CryptexCategory,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            function_name,
            branding_name,
            pseudo_code,
            rust_impl: None,
            python_impl: None,
            category,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Exploit metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExploitMetadata {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub references: Vec<String>,
    pub devices: Vec<String>,
    pub category: String,
    pub protocol: Protocol,
    pub severity: Severity,
    pub verified: bool,
    pub cryptex_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    HTTP,
    HTTPS,
    SSH,
    FTP,
    FTPS,
    Telnet,
    SNMP,
    TCP,
    UDP,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: Uuid,
    pub target: String,
    pub scan_type: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: ScanStatus,
    pub vulnerabilities: Vec<VulnerabilityFinding>,
    pub credentials: Vec<CredentialFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityFinding {
    pub exploit_id: Uuid,
    pub vulnerable: bool,
    pub proof: Option<String>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialFinding {
    pub username: String,
    pub password: String,
    pub protocol: Protocol,
    pub verified: bool,
}

/// QKD encryption metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QKDSession {
    pub id: Uuid,
    pub algorithm: String,
    pub key_material: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Training model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingModel {
    pub id: Uuid,
    pub name: String,
    pub model_type: ModelType,
    pub training_data_hash: String,
    pub accuracy: f64,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    VulnerabilityDetection,
    ExploitClassification,
    PayloadGeneration,
    AnomalyDetection,
}

/// Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_host: String,
    pub api_port: u16,
    pub db_path: String,
    pub max_threads: usize,
    pub timeout_seconds: u64,
    pub enable_qkd: bool,
    pub enable_python_compat: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_host: "127.0.0.1".to_string(),
            api_port: 8080,
            db_path: "./pyroutersploit.redb".to_string(),
            max_threads: num_cpus::get(),
            timeout_seconds: 30,
            enable_qkd: true,
            enable_python_compat: true,
        }
    }
}
