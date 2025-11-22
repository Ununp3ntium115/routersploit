// Scanner engine

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub threads: usize,
    pub timeout: u64,
}

// Scanner implementation will be added here
