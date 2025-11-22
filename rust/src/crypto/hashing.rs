// Comprehensive Multi-Algorithm Hashing Module
// Supports SHA-2, SHA-3, BLAKE2, BLAKE3, MD5, RIPEMD, and more

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fmt;

// Import all hashing algorithms
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest as Sha2Digest};
use sha3::{
    Sha3_224, Sha3_256, Sha3_384, Sha3_512,
    Shake128, Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};
use blake2::{Blake2b512, Blake2s256, Digest as Blake2Digest};
use blake3::Hasher as Blake3Hasher;
use md5::Md5;
use ripemd::Ripemd160;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HashAlgorithm {
    // SHA-2 family
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,

    // SHA-3 family
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,

    // SHAKE (extendable output functions)
    SHAKE128,
    SHAKE256,

    // BLAKE family
    BLAKE2b,
    BLAKE2s,
    BLAKE3,

    // Legacy (for compatibility)
    MD5,
    RIPEMD160,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HashAlgorithm {
    /// Get all available hash algorithms
    pub fn all() -> Vec<HashAlgorithm> {
        vec![
            HashAlgorithm::SHA224,
            HashAlgorithm::SHA256,
            HashAlgorithm::SHA384,
            HashAlgorithm::SHA512,
            HashAlgorithm::SHA512_224,
            HashAlgorithm::SHA512_256,
            HashAlgorithm::SHA3_224,
            HashAlgorithm::SHA3_256,
            HashAlgorithm::SHA3_384,
            HashAlgorithm::SHA3_512,
            HashAlgorithm::SHAKE128,
            HashAlgorithm::SHAKE256,
            HashAlgorithm::BLAKE2b,
            HashAlgorithm::BLAKE2s,
            HashAlgorithm::BLAKE3,
            HashAlgorithm::MD5,
            HashAlgorithm::RIPEMD160,
        ]
    }

    /// Get output size in bytes (None for variable-length)
    pub fn output_size(&self) -> Option<usize> {
        match self {
            HashAlgorithm::SHA224 => Some(28),
            HashAlgorithm::SHA256 => Some(32),
            HashAlgorithm::SHA384 => Some(48),
            HashAlgorithm::SHA512 => Some(64),
            HashAlgorithm::SHA512_224 => Some(28),
            HashAlgorithm::SHA512_256 => Some(32),
            HashAlgorithm::SHA3_224 => Some(28),
            HashAlgorithm::SHA3_256 => Some(32),
            HashAlgorithm::SHA3_384 => Some(48),
            HashAlgorithm::SHA3_512 => Some(64),
            HashAlgorithm::SHAKE128 => None, // Variable
            HashAlgorithm::SHAKE256 => None, // Variable
            HashAlgorithm::BLAKE2b => Some(64),
            HashAlgorithm::BLAKE2s => Some(32),
            HashAlgorithm::BLAKE3 => Some(32),
            HashAlgorithm::MD5 => Some(16),
            HashAlgorithm::RIPEMD160 => Some(20),
        }
    }
}

/// Hash result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResult {
    pub algorithm: HashAlgorithm,
    pub hash: Vec<u8>,
    pub hex: String,
}

impl HashResult {
    pub fn new(algorithm: HashAlgorithm, hash: Vec<u8>) -> Self {
        let hex = hex::encode(&hash);
        Self {
            algorithm,
            hash,
            hex,
        }
    }
}

/// Multi-algorithm hasher
pub struct MultiHasher;

impl MultiHasher {
    /// Hash data with specified algorithm
    pub fn hash(algorithm: HashAlgorithm, data: &[u8]) -> Result<HashResult> {
        let hash = match algorithm {
            // SHA-2 family
            HashAlgorithm::SHA224 => {
                let mut hasher = Sha224::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA384 => {
                let mut hasher = Sha384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA512 => {
                let mut hasher = Sha512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA512_224 => {
                let mut hasher = Sha512_224::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA512_256 => {
                let mut hasher = Sha512_256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }

            // SHA-3 family
            HashAlgorithm::SHA3_224 => {
                let mut hasher = Sha3_224::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA3_256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA3_384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::SHA3_512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }

            // SHAKE (using 256-bit output as default)
            HashAlgorithm::SHAKE128 => {
                let mut hasher = Shake128::default();
                hasher.update(data);
                let mut output = vec![0u8; 32];
                hasher.finalize_xof().read(&mut output);
                output
            }
            HashAlgorithm::SHAKE256 => {
                let mut hasher = Shake256::default();
                hasher.update(data);
                let mut output = vec![0u8; 64];
                hasher.finalize_xof().read(&mut output);
                output
            }

            // BLAKE family
            HashAlgorithm::BLAKE2b => {
                let mut hasher = Blake2b512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::BLAKE2s => {
                let mut hasher = Blake2s256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::BLAKE3 => {
                let mut hasher = Blake3Hasher::new();
                hasher.update(data);
                hasher.finalize().as_bytes().to_vec()
            }

            // Legacy
            HashAlgorithm::MD5 => {
                let mut hasher = Md5::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::RIPEMD160 => {
                let mut hasher = Ripemd160::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        };

        Ok(HashResult::new(algorithm, hash))
    }

    /// Hash data with all available algorithms
    pub fn hash_all(data: &[u8]) -> Result<Vec<HashResult>> {
        let algorithms = HashAlgorithm::all();
        let mut results = Vec::new();

        for algo in algorithms {
            results.push(Self::hash(algo, data)?);
        }

        Ok(results)
    }

    /// Hash string with specified algorithm
    pub fn hash_string(algorithm: HashAlgorithm, s: &str) -> Result<HashResult> {
        Self::hash(algorithm, s.as_bytes())
    }

    /// Hash file with specified algorithm
    pub fn hash_file(algorithm: HashAlgorithm, path: &std::path::Path) -> Result<HashResult> {
        let data = std::fs::read(path)?;
        Self::hash(algorithm, &data)
    }

    /// Verify hash
    pub fn verify(algorithm: HashAlgorithm, data: &[u8], expected_hash: &str) -> Result<bool> {
        let result = Self::hash(algorithm, data)?;
        Ok(result.hex.eq_ignore_ascii_case(expected_hash))
    }
}

/// HMAC (Hash-based Message Authentication Code) support
pub mod hmac {
    use super::*;
    use hmac::{Hmac, Mac};
    use sha2::{Sha256, Sha512};

    type HmacSha256 = Hmac<Sha256>;
    type HmacSha512 = Hmac<Sha512>;

    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| anyhow!("Invalid key length: {}", e))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    pub fn hmac_sha512(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let mut mac = HmacSha512::new_from_slice(key)
            .map_err(|e| anyhow!("Invalid key length: {}", e))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    pub fn verify_hmac_sha256(key: &[u8], data: &[u8], expected: &[u8]) -> Result<bool> {
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| anyhow!("Invalid key length: {}", e))?;
        mac.update(data);
        Ok(mac.verify_slice(expected).is_ok())
    }
}

/// Password hashing with Argon2
pub mod password {
    use super::*;
    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString
        },
        Argon2
    };

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("Password hashing failed: {}", e))?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow!("Invalid hash format: {}", e))?;
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_algorithms() {
        let data = b"Hello, PyRouterSploit!";
        let results = MultiHasher::hash_all(data).unwrap();

        // Should have one result per algorithm
        assert_eq!(results.len(), HashAlgorithm::all().len());

        // Check each result has correct structure
        for result in results {
            assert!(!result.hash.is_empty());
            assert!(!result.hex.is_empty());
            assert_eq!(result.hex.len(), result.hash.len() * 2);
        }
    }

    #[test]
    fn test_sha256() {
        let data = b"test";
        let result = MultiHasher::hash(HashAlgorithm::SHA256, data).unwrap();
        assert_eq!(
            result.hex,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn test_blake3() {
        let data = b"test";
        let result = MultiHasher::hash(HashAlgorithm::BLAKE3, data).unwrap();
        assert_eq!(result.hash.len(), 32);
    }

    #[test]
    fn test_verify() {
        let data = b"test";
        let expected = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        assert!(MultiHasher::verify(HashAlgorithm::SHA256, data, expected).unwrap());
    }
}
