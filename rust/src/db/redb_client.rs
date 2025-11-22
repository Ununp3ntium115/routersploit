// redb Database Client for PyRouterSploit

use anyhow::{Result, Context};
use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::db::models::*;

// Table definitions
const CRYPTEX_TABLE: TableDefinition<&str, &str> = TableDefinition::new("cryptex");
const EXPLOITS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("exploits");
const SCANS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("scans");
const QKD_SESSIONS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("qkd_sessions");
const MODELS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("training_models");
const CONFIG_TABLE: TableDefinition<&str, &str> = TableDefinition::new("config");

lazy_static! {
    static ref DB: RwLock<Option<Arc<Database>>> = RwLock::new(None);
}

/// Initialize the database
pub async fn init_database() -> Result<()> {
    let db_path = get_db_path();
    let db = Database::create(&db_path)
        .context("Failed to create database")?;

    // Initialize tables
    let write_txn = db.begin_write()?;
    {
        let _ = write_txn.open_table(CRYPTEX_TABLE)?;
        let _ = write_txn.open_table(EXPLOITS_TABLE)?;
        let _ = write_txn.open_table(SCANS_TABLE)?;
        let _ = write_txn.open_table(QKD_SESSIONS_TABLE)?;
        let _ = write_txn.open_table(MODELS_TABLE)?;
        let _ = write_txn.open_table(CONFIG_TABLE)?;
    }
    write_txn.commit()?;

    *DB.write() = Some(Arc::new(db));

    tracing::info!("Database initialized at {}", db_path);
    Ok(())
}

/// Close the database
pub async fn close_database() -> Result<()> {
    *DB.write() = None;
    tracing::info!("Database closed");
    Ok(())
}

fn get_db_path() -> String {
    std::env::var("PYROUTERSPLOIT_DB_PATH")
        .unwrap_or_else(|_| "./data/pyroutersploit.redb".to_string())
}

fn get_db() -> Result<Arc<Database>> {
    DB.read()
        .as_ref()
        .cloned()
        .context("Database not initialized")
}

/// Cryptex operations
pub mod cryptex {
    use super::*;

    pub fn insert(entry: &CryptexEntry) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(CRYPTEX_TABLE)?;
            let key = entry.id.to_string();
            let value = serde_json::to_string(entry)?;
            table.insert(key.as_str(), value.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_by_id(id: &Uuid) -> Result<Option<CryptexEntry>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CRYPTEX_TABLE)?;

        let key = id.to_string();
        let value = table.get(key.as_str())?;

        match value {
            Some(v) => {
                let entry: CryptexEntry = serde_json::from_str(v.value())?;
                Ok(Some(entry))
            }
            None => Ok(None),
        }
    }

    pub fn get_by_function_name(function_name: &str) -> Result<Option<CryptexEntry>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CRYPTEX_TABLE)?;

        for item in table.iter()? {
            let (_, value) = item?;
            let entry: CryptexEntry = serde_json::from_str(value.value())?;
            if entry.function_name == function_name {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    pub fn get_by_branding_name(branding_name: &str) -> Result<Option<CryptexEntry>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CRYPTEX_TABLE)?;

        for item in table.iter()? {
            let (_, value) = item?;
            let entry: CryptexEntry = serde_json::from_str(value.value())?;
            if entry.branding_name == branding_name {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    pub fn list_all() -> Result<Vec<CryptexEntry>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CRYPTEX_TABLE)?;

        let mut entries = Vec::new();
        for item in table.iter()? {
            let (_, value) = item?;
            let entry: CryptexEntry = serde_json::from_str(value.value())?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub fn search(query: &str) -> Result<Vec<CryptexEntry>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CRYPTEX_TABLE)?;

        let query_lower = query.to_lowercase();
        let mut entries = Vec::new();

        for item in table.iter()? {
            let (_, value) = item?;
            let entry: CryptexEntry = serde_json::from_str(value.value())?;

            if entry.function_name.to_lowercase().contains(&query_lower)
                || entry.branding_name.to_lowercase().contains(&query_lower)
                || entry.pseudo_code.to_lowercase().contains(&query_lower)
            {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    pub fn delete(id: &Uuid) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(CRYPTEX_TABLE)?;
            let key = id.to_string();
            table.remove(key.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }
}

/// Exploit operations
pub mod exploits {
    use super::*;

    pub fn insert(metadata: &ExploitMetadata) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(EXPLOITS_TABLE)?;
            let key = metadata.id.to_string();
            let value = serde_json::to_string(metadata)?;
            table.insert(key.as_str(), value.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_by_id(id: &Uuid) -> Result<Option<ExploitMetadata>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(EXPLOITS_TABLE)?;

        let key = id.to_string();
        let value = table.get(key.as_str())?;

        match value {
            Some(v) => {
                let metadata: ExploitMetadata = serde_json::from_str(v.value())?;
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    pub fn list_all() -> Result<Vec<ExploitMetadata>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(EXPLOITS_TABLE)?;

        let mut metadata_list = Vec::new();
        for item in table.iter()? {
            let (_, value) = item?;
            let metadata: ExploitMetadata = serde_json::from_str(value.value())?;
            metadata_list.push(metadata);
        }
        Ok(metadata_list)
    }
}

/// Scan operations
pub mod scans {
    use super::*;

    pub fn insert(result: &ScanResult) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(SCANS_TABLE)?;
            let key = result.id.to_string();
            let value = serde_json::to_string(result)?;
            table.insert(key.as_str(), value.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_by_id(id: &Uuid) -> Result<Option<ScanResult>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(SCANS_TABLE)?;

        let key = id.to_string();
        let value = table.get(key.as_str())?;

        match value {
            Some(v) => {
                let result: ScanResult = serde_json::from_str(v.value())?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    pub fn update(result: &ScanResult) -> Result<()> {
        insert(result)
    }
}

/// QKD session operations
pub mod qkd_sessions {
    use super::*;

    pub fn insert(session: &QKDSession) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(QKD_SESSIONS_TABLE)?;
            let key = session.id.to_string();
            let value = serde_json::to_string(session)?;
            table.insert(key.as_str(), value.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_by_id(id: &Uuid) -> Result<Option<QKDSession>> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(QKD_SESSIONS_TABLE)?;

        let key = id.to_string();
        let value = table.get(key.as_str())?;

        match value {
            Some(v) => {
                let session: QKDSession = serde_json::from_str(v.value())?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }
}

/// Configuration operations
pub mod config {
    use super::*;

    const CONFIG_KEY: &str = "main_config";

    pub fn save(config: &Config) -> Result<()> {
        let db = get_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(CONFIG_TABLE)?;
            let value = serde_json::to_string(config)?;
            table.insert(CONFIG_KEY, value.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn load() -> Result<Config> {
        let db = get_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(CONFIG_TABLE)?;

        let value = table.get(CONFIG_KEY)?;

        match value {
            Some(v) => {
                let config: Config = serde_json::from_str(v.value())?;
                Ok(config)
            }
            None => Ok(Config::default()),
        }
    }
}
