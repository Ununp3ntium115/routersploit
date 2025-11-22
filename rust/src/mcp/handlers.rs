// MCP Request Handlers

use anyhow::{Result, anyhow};
use serde_json::{json, Value};

use crate::db::{CryptexDictionary, models::CryptexCategory};
use crate::crypto::{HashAlgorithm, MultiHasher, QKDEncryption};
use super::schema::*;

pub struct MCPHandlers;

impl MCPHandlers {
    /// List all available tools
    pub fn list_tools() -> Result<Value> {
        let tools = vec![
            MCPTool {
                name: "cryptex_query".to_string(),
                description: "Query the cryptex dictionary by function name, branding name, or search term".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "function_name": {
                            "type": "string",
                            "description": "Search by function name"
                        },
                        "branding_name": {
                            "type": "string",
                            "description": "Search by branding name (e.g., pyroutersploit_*)"
                        },
                        "search": {
                            "type": "string",
                            "description": "Free-text search across all fields"
                        },
                        "category": {
                            "type": "string",
                            "enum": ["Exploit", "Scanner", "Credential", "Payload", "Encoder", "Utility"],
                            "description": "Filter by category"
                        }
                    }
                }),
            },
            MCPTool {
                name: "cryptex_add".to_string(),
                description: "Add a new entry to the cryptex dictionary".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "function_name": {
                            "type": "string",
                            "description": "Internal function name"
                        },
                        "branding_name": {
                            "type": "string",
                            "description": "PyRouterSploit branding name (e.g., pyroutersploit_dlink_pwn)"
                        },
                        "pseudo_code": {
                            "type": "string",
                            "description": "Human-readable description/pseudo-code"
                        },
                        "category": {
                            "type": "string",
                            "enum": ["Exploit", "Scanner", "Credential", "Payload", "Encoder", "Utility"]
                        },
                        "rust_impl": {
                            "type": "string",
                            "description": "Rust implementation path (optional)"
                        },
                        "python_impl": {
                            "type": "string",
                            "description": "Python implementation path (optional)"
                        }
                    },
                    "required": ["function_name", "branding_name", "pseudo_code", "category"]
                }),
            },
            MCPTool {
                name: "list_exploits".to_string(),
                description: "List all available exploits".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            MCPTool {
                name: "run_exploit".to_string(),
                description: "Execute an exploit against a target".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "exploit_id": {
                            "type": "string",
                            "description": "Exploit UUID or function name"
                        },
                        "branding_name": {
                            "type": "string",
                            "description": "Branding name (alternative to exploit_id)"
                        },
                        "target": {
                            "type": "string",
                            "description": "Target IP address or hostname"
                        },
                        "options": {
                            "type": "object",
                            "description": "Additional exploit options"
                        }
                    },
                    "required": ["target"]
                }),
            },
            MCPTool {
                name: "scan_target".to_string(),
                description: "Scan a target for vulnerabilities".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target": {
                            "type": "string",
                            "description": "Target IP address or hostname"
                        },
                        "scan_type": {
                            "type": "string",
                            "enum": ["autopwn", "http", "ssh", "telnet", "all"],
                            "description": "Type of scan to perform"
                        },
                        "threads": {
                            "type": "integer",
                            "description": "Number of threads (default: 10)"
                        }
                    },
                    "required": ["target"]
                }),
            },
            MCPTool {
                name: "multi_hash".to_string(),
                description: "Hash data with one or all available algorithms (SHA-2/3, BLAKE, MD5, etc.)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "data": {
                            "type": "string",
                            "description": "Data to hash"
                        },
                        "algorithm": {
                            "type": "string",
                            "enum": ["SHA224", "SHA256", "SHA384", "SHA512", "SHA3_256", "SHA3_512", "BLAKE2b", "BLAKE2s", "BLAKE3", "MD5", "RIPEMD160"],
                            "description": "Specific algorithm (optional)"
                        },
                        "all_algorithms": {
                            "type": "boolean",
                            "description": "Hash with all algorithms"
                        }
                    },
                    "required": ["data"]
                }),
            },
            MCPTool {
                name: "qkd_encrypt".to_string(),
                description: "Encrypt data using Quantum Key Distribution (QKD) encryption".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "data": {
                            "type": "string",
                            "description": "Data to encrypt"
                        },
                        "key_size": {
                            "type": "integer",
                            "description": "Key size in bytes (default: 32)"
                        }
                    },
                    "required": ["data"]
                }),
            },
        ];

        Ok(json!({ "tools": tools }))
    }

    /// Handle cryptex query
    pub fn handle_cryptex_query(params: CryptexQueryParams) -> Result<Value> {
        if let Some(function_name) = params.function_name {
            let entry = CryptexDictionary::lookup_function(&function_name)?;
            return Ok(json!({ "result": entry }));
        }

        if let Some(branding_name) = params.branding_name {
            let entry = CryptexDictionary::lookup_branding(&branding_name)?;
            return Ok(json!({ "result": entry }));
        }

        if let Some(search) = params.search {
            let results = CryptexDictionary::search(&search)?;
            return Ok(json!({ "results": results, "count": results.len() }));
        }

        if let Some(category_str) = params.category {
            let category = match category_str.as_str() {
                "Exploit" => CryptexCategory::Exploit,
                "Scanner" => CryptexCategory::Scanner,
                "Credential" => CryptexCategory::Credential,
                "Payload" => CryptexCategory::Payload,
                "Encoder" => CryptexCategory::Encoder,
                "Utility" => CryptexCategory::Utility,
                _ => return Err(anyhow!("Invalid category")),
            };
            let results = CryptexDictionary::list_by_category(category)?;
            return Ok(json!({ "results": results, "count": results.len() }));
        }

        // List all if no specific query
        let all = CryptexDictionary::list_all()?;
        Ok(json!({ "results": all, "count": all.len() }))
    }

    /// Handle cryptex add entry
    pub fn handle_cryptex_add(params: CryptexAddParams) -> Result<Value> {
        let category = match params.category.as_str() {
            "Exploit" => CryptexCategory::Exploit,
            "Scanner" => CryptexCategory::Scanner,
            "Credential" => CryptexCategory::Credential,
            "Payload" => CryptexCategory::Payload,
            "Encoder" => CryptexCategory::Encoder,
            "Utility" => CryptexCategory::Utility,
            _ => return Err(anyhow!("Invalid category")),
        };

        let entry = CryptexDictionary::add_entry_with_impl(
            params.function_name,
            params.branding_name,
            params.pseudo_code,
            category,
            params.rust_impl,
            params.python_impl,
        )?;

        Ok(json!({
            "success": true,
            "entry": entry,
            "message": "Cryptex entry added successfully"
        }))
    }

    /// Handle list exploits
    pub fn handle_list_exploits() -> Result<Value> {
        use crate::db::redb_client::exploits;

        let exploits = exploits::list_all()?;
        Ok(json!({
            "exploits": exploits,
            "count": exploits.len()
        }))
    }

    /// Handle run exploit
    pub fn handle_run_exploit(params: ExploitRunParams) -> Result<Value> {
        // This is a placeholder - actual implementation would execute the exploit
        Ok(json!({
            "success": true,
            "message": "Exploit execution not yet fully implemented",
            "target": params.target,
            "note": "This is a framework stub - will be implemented with full exploit engine"
        }))
    }

    /// Handle scan target
    pub fn handle_scan_target(params: ScanParams) -> Result<Value> {
        Ok(json!({
            "success": true,
            "message": "Scan initiated",
            "target": params.target,
            "scan_type": params.scan_type.unwrap_or("autopwn".to_string()),
            "note": "Scanner engine will be fully implemented in next phase"
        }))
    }

    /// Handle multi-hash
    pub fn handle_multi_hash(params: HashParams) -> Result<Value> {
        if params.all_algorithms {
            let results = MultiHasher::hash_all(params.data.as_bytes())?;
            let hashes: Vec<_> = results.iter().map(|r| {
                json!({
                    "algorithm": r.algorithm.to_string(),
                    "hash": r.hex
                })
            }).collect();
            return Ok(json!({
                "data": params.data,
                "hashes": hashes,
                "count": hashes.len()
            }));
        }

        let algorithm = if let Some(algo_str) = params.algorithm {
            match algo_str.to_uppercase().as_str() {
                "SHA224" => HashAlgorithm::SHA224,
                "SHA256" => HashAlgorithm::SHA256,
                "SHA384" => HashAlgorithm::SHA384,
                "SHA512" => HashAlgorithm::SHA512,
                "SHA3_256" => HashAlgorithm::SHA3_256,
                "SHA3_512" => HashAlgorithm::SHA3_512,
                "BLAKE2B" => HashAlgorithm::BLAKE2b,
                "BLAKE2S" => HashAlgorithm::BLAKE2s,
                "BLAKE3" => HashAlgorithm::BLAKE3,
                "MD5" => HashAlgorithm::MD5,
                "RIPEMD160" => HashAlgorithm::RIPEMD160,
                _ => HashAlgorithm::SHA256,
            }
        } else {
            HashAlgorithm::SHA256
        };

        let result = MultiHasher::hash(algorithm, params.data.as_bytes())?;
        Ok(json!({
            "data": params.data,
            "algorithm": result.algorithm.to_string(),
            "hash": result.hex
        }))
    }

    /// Handle QKD encryption
    pub fn handle_qkd_encrypt(params: QKDEncryptParams) -> Result<Value> {
        let key_size = params.key_size.unwrap_or(32);
        let qkd = QKDEncryption::new_session(key_size)?;
        let ciphertext = qkd.encrypt(params.data.as_bytes())?;
        let ciphertext_hex = hex::encode(&ciphertext);

        qkd.save_session()?;

        Ok(json!({
            "success": true,
            "ciphertext": ciphertext_hex,
            "session_id": qkd.session_id,
            "key_size": key_size,
            "message": "Data encrypted with QKD-derived key"
        }))
    }
}
