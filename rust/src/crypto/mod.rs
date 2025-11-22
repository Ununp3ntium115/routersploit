pub mod hashing;
pub mod qkd;
pub mod pqc;

pub use hashing::{HashAlgorithm, HashResult, MultiHasher};
pub use qkd::{QKDProtocol, QKDKeyGenerator, QKDEncryption};

use anyhow::Result;

/// Initialize the crypto subsystem
pub fn init() -> Result<()> {
    tracing::info!("Crypto subsystem initialized");
    Ok(())
}
