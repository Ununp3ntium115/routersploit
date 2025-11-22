// Payload generation module

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Architecture {
    ARMLE,
    MIPSBE,
    MIPSLE,
    X86,
    X64,
    PHP,
    Python,
    Perl,
}

// Payload generation will be added here
