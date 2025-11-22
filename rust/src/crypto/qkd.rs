// Quantum Key Distribution (QKD) and Post-Quantum Cryptography Module

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rand::Rng;

use crate::db::models::QKDSession;
use crate::crypto::hashing::{HashAlgorithm, MultiHasher};

/// QKD Protocol types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QKDProtocol {
    /// BB84 Protocol (Bennett-Brassard 1984)
    BB84,
    /// Decoy State QKD
    DecoyState,
    /// Continuous Variable QKD
    ContinuousVariable,
    /// Device-Independent QKD
    DeviceIndependent,
}

/// Quantum state representation (simplified simulation)
#[derive(Debug, Clone, Copy)]
pub enum QuantumBasis {
    Rectilinear, // + basis (0° and 90°)
    Diagonal,    // × basis (45° and 135°)
}

#[derive(Debug, Clone, Copy)]
pub struct QuantumBit {
    basis: QuantumBasis,
    value: bool,
}

/// BB84 QKD Protocol Implementation (Simulated)
pub struct BB84Simulator {
    bits_count: usize,
}

impl BB84Simulator {
    pub fn new(bits_count: usize) -> Self {
        Self { bits_count }
    }

    /// Alice prepares quantum states
    pub fn alice_prepare(&self) -> (Vec<QuantumBit>, Vec<QuantumBasis>) {
        let mut rng = rand::thread_rng();
        let mut bits = Vec::new();
        let mut bases = Vec::new();

        for _ in 0..self.bits_count {
            let basis = if rng.gen_bool(0.5) {
                QuantumBasis::Rectilinear
            } else {
                QuantumBasis::Diagonal
            };
            let value = rng.gen_bool(0.5);

            bits.push(QuantumBit { basis, value });
            bases.push(basis);
        }

        (bits, bases)
    }

    /// Bob measures quantum states
    pub fn bob_measure(&self, alice_bits: &[QuantumBit]) -> (Vec<bool>, Vec<QuantumBasis>) {
        let mut rng = rand::thread_rng();
        let mut measured_values = Vec::new();
        let mut bob_bases = Vec::new();

        for qubit in alice_bits {
            let bob_basis = if rng.gen_bool(0.5) {
                QuantumBasis::Rectilinear
            } else {
                QuantumBasis::Diagonal
            };

            // If bases match, Bob gets correct value
            // If bases don't match, Bob gets random value
            let measured_value = if std::mem::discriminant(&qubit.basis)
                == std::mem::discriminant(&bob_basis)
            {
                qubit.value
            } else {
                rng.gen_bool(0.5)
            };

            measured_values.push(measured_value);
            bob_bases.push(bob_basis);
        }

        (measured_values, bob_bases)
    }

    /// Sift key - keep only bits where bases match
    pub fn sift_key(
        alice_bases: &[QuantumBasis],
        bob_bases: &[QuantumBasis],
        bob_values: &[bool],
    ) -> Vec<bool> {
        alice_bases
            .iter()
            .zip(bob_bases)
            .zip(bob_values)
            .filter_map(|((alice_basis, bob_basis), value)| {
                if std::mem::discriminant(alice_basis) == std::mem::discriminant(bob_basis) {
                    Some(*value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Error correction and privacy amplification (simplified)
    pub fn finalize_key(sifted_key: Vec<bool>, target_size: usize) -> Vec<u8> {
        // Convert bits to bytes
        let mut key_bytes = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for bit in sifted_key.iter().take(target_size * 8) {
            current_byte = (current_byte << 1) | (*bit as u8);
            bit_count += 1;

            if bit_count == 8 {
                key_bytes.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }

        // Pad if necessary
        while key_bytes.len() < target_size {
            key_bytes.push(0);
        }

        key_bytes.truncate(target_size);
        key_bytes
    }
}

/// QKD Key Generator
pub struct QKDKeyGenerator;

impl QKDKeyGenerator {
    /// Generate a quantum-safe key using BB84 protocol simulation
    pub fn generate_bb84_key(key_size: usize) -> Result<Vec<u8>> {
        // Generate extra bits to account for basis mismatch
        let simulator = BB84Simulator::new(key_size * 16);

        // Alice prepares
        let (alice_bits, alice_bases) = simulator.alice_prepare();

        // Bob measures
        let (bob_values, bob_bases) = simulator.bob_measure(&alice_bits);

        // Sift key
        let sifted_key = BB84Simulator::sift_key(&alice_bases, &bob_bases, &bob_values);

        // Finalize
        Ok(BB84Simulator::finalize_key(sifted_key, key_size))
    }

    /// Generate hybrid classical-quantum key
    pub fn generate_hybrid_key(key_size: usize) -> Result<Vec<u8>> {
        let qkd_key = Self::generate_bb84_key(key_size)?;

        // Mix with classical entropy
        let mut rng = rand::thread_rng();
        let classical_entropy: Vec<u8> = (0..key_size).map(|_| rng.gen()).collect();

        // XOR combine
        let hybrid_key: Vec<u8> = qkd_key
            .iter()
            .zip(classical_entropy.iter())
            .map(|(q, c)| q ^ c)
            .collect();

        // Hash for additional mixing
        let result = MultiHasher::hash(HashAlgorithm::BLAKE3, &hybrid_key)?;
        Ok(result.hash[..key_size.min(32)].to_vec())
    }
}

/// QKD Encryption Engine
pub struct QKDEncryption {
    session_id: Uuid,
    key_material: Vec<u8>,
}

impl QKDEncryption {
    /// Create new QKD encryption session
    pub fn new_session(key_size: usize) -> Result<Self> {
        let key_material = QKDKeyGenerator::generate_hybrid_key(key_size)?;

        Ok(Self {
            session_id: Uuid::new_v4(),
            key_material,
        })
    }

    /// Encrypt data with quantum-derived key
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        // Use ChaCha20-Poly1305 (quantum-resistant symmetric cipher)
        use chacha20poly1305::{
            aead::{Aead, KeyInit, OsRng},
            ChaCha20Poly1305, Nonce,
        };

        // Derive key from QKD material
        let key_bytes = if self.key_material.len() >= 32 {
            &self.key_material[..32]
        } else {
            // Hash to get 32 bytes
            let hash = MultiHasher::hash(HashAlgorithm::SHA256, &self.key_material)?;
            return self.encrypt_with_key(&hash.hash, plaintext);
        };

        self.encrypt_with_key(key_bytes, plaintext)
    }

    fn encrypt_with_key(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| anyhow!("Key error: {}", e))?;

        // Generate random nonce
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data with quantum-derived key
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        if ciphertext.len() < 12 {
            return Err(anyhow!("Ciphertext too short"));
        }

        // Extract nonce
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Derive key
        let key_bytes = if self.key_material.len() >= 32 {
            &self.key_material[..32]
        } else {
            let hash = MultiHasher::hash(HashAlgorithm::SHA256, &self.key_material)?;
            return self.decrypt_with_key(&hash.hash, encrypted_data, nonce);
        };

        self.decrypt_with_key(key_bytes, encrypted_data, nonce)
    }

    fn decrypt_with_key(&self, key: &[u8], ciphertext: &[u8], nonce: &Nonce) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305,
        };

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| anyhow!("Key error: {}", e))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    /// Save session to database
    pub fn save_session(&self) -> Result<()> {
        let session = QKDSession {
            id: self.session_id,
            algorithm: "BB84-Hybrid".to_string(),
            key_material: self.key_material.clone(),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
        };

        crate::db::redb_client::qkd_sessions::insert(&session)?;
        Ok(())
    }

    /// Load session from database
    pub fn load_session(session_id: &Uuid) -> Result<Self> {
        let session = crate::db::redb_client::qkd_sessions::get_by_id(session_id)?
            .ok_or_else(|| anyhow!("Session not found"))?;

        Ok(Self {
            session_id: session.id,
            key_material: session.key_material,
        })
    }
}

/// Post-Quantum Cryptography wrapper
pub mod pqc {
    use super::*;

    /// Kyber key encapsulation
    pub mod kyber {
        use pqcrypto_kyber::kyber1024::*;
        use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret, Ciphertext};
        use anyhow::Result;

        pub struct KyberKEM {
            public_key: Vec<u8>,
            secret_key: Vec<u8>,
        }

        impl KyberKEM {
            pub fn generate() -> Self {
                let (pk, sk) = keypair();
                Self {
                    public_key: pk.as_bytes().to_vec(),
                    secret_key: sk.as_bytes().to_vec(),
                }
            }

            pub fn encapsulate(&self) -> Result<(Vec<u8>, Vec<u8>)> {
                let pk = PublicKey::from_bytes(&self.public_key)
                    .map_err(|e| anyhow::anyhow!("Invalid public key: {:?}", e))?;
                let (ss, ct) = encapsulate(&pk);
                Ok((ss.as_bytes().to_vec(), ct.as_bytes().to_vec()))
            }

            pub fn decapsulate(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
                let sk = SecretKey::from_bytes(&self.secret_key)
                    .map_err(|e| anyhow::anyhow!("Invalid secret key: {:?}", e))?;
                let ct = Ciphertext::from_bytes(ciphertext)
                    .map_err(|e| anyhow::anyhow!("Invalid ciphertext: {:?}", e))?;
                let ss = decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            }
        }
    }

    /// Dilithium digital signatures
    pub mod dilithium {
        use pqcrypto_dilithium::dilithium5::*;
        use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage, DetachedSignature};
        use anyhow::Result;

        pub struct DilithiumSignature {
            public_key: Vec<u8>,
            secret_key: Vec<u8>,
        }

        impl DilithiumSignature {
            pub fn generate() -> Self {
                let (pk, sk) = keypair();
                Self {
                    public_key: pk.as_bytes().to_vec(),
                    secret_key: sk.as_bytes().to_vec(),
                }
            }

            pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
                let sk = SecretKey::from_bytes(&self.secret_key)
                    .map_err(|e| anyhow::anyhow!("Invalid secret key: {:?}", e))?;
                let signed = sign(message, &sk);
                Ok(signed.as_bytes().to_vec())
            }

            pub fn verify(&self, signed_message: &[u8]) -> Result<Vec<u8>> {
                let pk = PublicKey::from_bytes(&self.public_key)
                    .map_err(|e| anyhow::anyhow!("Invalid public key: {:?}", e))?;
                let sm = SignedMessage::from_bytes(signed_message)
                    .map_err(|e| anyhow::anyhow!("Invalid signed message: {:?}", e))?;
                let message = open(&sm, &pk)
                    .map_err(|e| anyhow::anyhow!("Verification failed: {:?}", e))?;
                Ok(message.to_vec())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bb84_simulation() {
        let key = QKDKeyGenerator::generate_bb84_key(32).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_qkd_encryption() {
        let qkd = QKDEncryption::new_session(32).unwrap();
        let plaintext = b"Hello, Quantum World!";

        let ciphertext = qkd.encrypt(plaintext).unwrap();
        let decrypted = qkd.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_kyber_kem() {
        let kem = pqc::kyber::KyberKEM::generate();
        let (shared_secret_1, ciphertext) = kem.encapsulate().unwrap();
        let shared_secret_2 = kem.decapsulate(&ciphertext).unwrap();
        assert_eq!(shared_secret_1, shared_secret_2);
    }

    #[test]
    fn test_dilithium_signature() {
        let sig = pqc::dilithium::DilithiumSignature::generate();
        let message = b"Test message";
        let signed = sig.sign(message).unwrap();
        let verified = sig.verify(&signed).unwrap();
        assert_eq!(message.to_vec(), verified);
    }
}
