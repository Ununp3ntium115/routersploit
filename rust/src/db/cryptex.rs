// Cryptex Dictionary - Function to Branding Name Mapping System

use anyhow::Result;
use uuid::Uuid;
use crate::db::models::{CryptexEntry, CryptexCategory};
use crate::db::redb_client::cryptex as db;

/// Cryptex Dictionary Manager
pub struct CryptexDictionary;

impl CryptexDictionary {
    /// Add a new cryptex entry
    pub fn add_entry(
        function_name: impl Into<String>,
        branding_name: impl Into<String>,
        pseudo_code: impl Into<String>,
        category: CryptexCategory,
    ) -> Result<CryptexEntry> {
        let entry = CryptexEntry::new(
            function_name.into(),
            branding_name.into(),
            pseudo_code.into(),
            category,
        );

        db::insert(&entry)?;
        Ok(entry)
    }

    /// Add entry with implementation paths
    pub fn add_entry_with_impl(
        function_name: impl Into<String>,
        branding_name: impl Into<String>,
        pseudo_code: impl Into<String>,
        category: CryptexCategory,
        rust_impl: Option<String>,
        python_impl: Option<String>,
    ) -> Result<CryptexEntry> {
        let mut entry = CryptexEntry::new(
            function_name.into(),
            branding_name.into(),
            pseudo_code.into(),
            category,
        );

        entry.rust_impl = rust_impl;
        entry.python_impl = python_impl;

        db::insert(&entry)?;
        Ok(entry)
    }

    /// Lookup by function name
    pub fn lookup_function(function_name: &str) -> Result<Option<CryptexEntry>> {
        db::get_by_function_name(function_name)
    }

    /// Lookup by branding name
    pub fn lookup_branding(branding_name: &str) -> Result<Option<CryptexEntry>> {
        db::get_by_branding_name(branding_name)
    }

    /// Search cryptex
    pub fn search(query: &str) -> Result<Vec<CryptexEntry>> {
        db::search(query)
    }

    /// List all entries
    pub fn list_all() -> Result<Vec<CryptexEntry>> {
        db::list_all()
    }

    /// List by category
    pub fn list_by_category(category: CryptexCategory) -> Result<Vec<CryptexEntry>> {
        let all = db::list_all()?;
        Ok(all.into_iter().filter(|e| e.category == category).collect())
    }

    /// Delete entry
    pub fn delete(id: &Uuid) -> Result<()> {
        db::delete(id)
    }

    /// Populate default pyroutersploit entries
    pub fn populate_defaults() -> Result<()> {
        tracing::info!("Populating default cryptex entries...");

        // Exploits
        Self::add_entry_with_impl(
            "exploit_dlink_rce_hnap",
            "pyroutersploit_dlink_hnap_pwn",
            "Execute remote code on D-Link routers via HNAP vulnerability",
            CryptexCategory::Exploit,
            Some("crate::core::exploit::dlink::hnap_rce".to_string()),
            Some("routersploit.modules.exploits.routers.dlink.hnap_login".to_string()),
        )?;

        Self::add_entry_with_impl(
            "exploit_linksys_wrt54gl_rce",
            "pyroutersploit_linksys_wrt54gl_exec",
            "Remote command execution on Linksys WRT54GL routers",
            CryptexCategory::Exploit,
            None,
            Some("routersploit.modules.exploits.routers.linksys.wrt54gl_exec".to_string()),
        )?;

        Self::add_entry_with_impl(
            "exploit_netgear_setup_rce",
            "pyroutersploit_netgear_unauth_exec",
            "Unauthenticated remote code execution on Netgear routers",
            CryptexCategory::Exploit,
            None,
            Some("routersploit.modules.exploits.routers.netgear.multi_rce".to_string()),
        )?;

        // Scanners
        Self::add_entry_with_impl(
            "scanner_router_autopwn",
            "pyroutersploit_autopwn_scanner",
            "Automated vulnerability scanner for routers across all protocols",
            CryptexCategory::Scanner,
            Some("crate::core::scanner::autopwn".to_string()),
            Some("routersploit.modules.scanners.autopwn".to_string()),
        )?;

        Self::add_entry_with_impl(
            "scanner_camera_vuln",
            "pyroutersploit_camera_scanner",
            "Vulnerability scanner targeting IP cameras",
            CryptexCategory::Scanner,
            None,
            Some("routersploit.modules.scanners.cameras".to_string()),
        )?;

        // Credentials
        Self::add_entry_with_impl(
            "creds_ssh_default",
            "pyroutersploit_ssh_bruteforce",
            "Test default and common SSH credentials",
            CryptexCategory::Credential,
            Some("crate::core::creds::ssh_default".to_string()),
            Some("routersploit.modules.creds.generic.ssh_default".to_string()),
        )?;

        Self::add_entry_with_impl(
            "creds_telnet_default",
            "pyroutersploit_telnet_bruteforce",
            "Test default and common Telnet credentials",
            CryptexCategory::Credential,
            None,
            Some("routersploit.modules.creds.generic.telnet_default".to_string()),
        )?;

        // Payloads
        Self::add_entry_with_impl(
            "payload_reverse_tcp_mipsle",
            "pyroutersploit_mipsle_revshell",
            "MIPS little-endian reverse TCP shell payload",
            CryptexCategory::Payload,
            Some("crate::core::payload::mipsle::reverse_tcp".to_string()),
            Some("routersploit.modules.payloads.mipsle.reverse_tcp".to_string()),
        )?;

        Self::add_entry_with_impl(
            "payload_reverse_tcp_armle",
            "pyroutersploit_armle_revshell",
            "ARM little-endian reverse TCP shell payload",
            CryptexCategory::Payload,
            Some("crate::core::payload::armle::reverse_tcp".to_string()),
            Some("routersploit.modules.payloads.armle.reverse_tcp".to_string()),
        )?;

        // Encoders
        Self::add_entry_with_impl(
            "encoder_php_base64",
            "pyroutersploit_php_b64_encoder",
            "Base64 encoder for PHP payloads",
            CryptexCategory::Encoder,
            Some("crate::core::encoder::php::base64".to_string()),
            Some("routersploit.modules.encoders.php.base64".to_string()),
        )?;

        // Utilities
        Self::add_entry_with_impl(
            "util_qkd_encrypt",
            "pyroutersploit_quantum_encrypt",
            "Quantum key distribution encryption utility",
            CryptexCategory::Utility,
            Some("crate::crypto::qkd::encrypt".to_string()),
            None,
        )?;

        Self::add_entry_with_impl(
            "util_multi_hash",
            "pyroutersploit_omni_hasher",
            "Multi-algorithm hashing utility (SHA-2/3, BLAKE, etc.)",
            CryptexCategory::Utility,
            Some("crate::crypto::hashing::multi_hash".to_string()),
            None,
        )?;

        tracing::info!("Default cryptex entries populated successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cryptex_operations() {
        // Initialize test database
        std::env::set_var("PYROUTERSPLOIT_DB_PATH", ":memory:");
        crate::db::redb_client::init_database().await.unwrap();

        // Add entry
        let entry = CryptexDictionary::add_entry(
            "test_function",
            "pyroutersploit_test",
            "Test pseudo code",
            CryptexCategory::Exploit,
        )
        .unwrap();

        // Lookup by function name
        let found = CryptexDictionary::lookup_function("test_function")
            .unwrap()
            .unwrap();
        assert_eq!(found.branding_name, "pyroutersploit_test");

        // Lookup by branding name
        let found = CryptexDictionary::lookup_branding("pyroutersploit_test")
            .unwrap()
            .unwrap();
        assert_eq!(found.function_name, "test_function");

        // Search
        let results = CryptexDictionary::search("test").unwrap();
        assert_eq!(results.len(), 1);

        // Delete
        CryptexDictionary::delete(&entry.id).unwrap();
        let not_found = CryptexDictionary::lookup_function("test_function").unwrap();
        assert!(not_found.is_none());
    }
}
