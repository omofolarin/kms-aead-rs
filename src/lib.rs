//! # KMS/AEAD envelope encryption for GCP/AWS KMS and Ring AEAD encryption
//!
//! Available providers:
//! - Google Cloud Platform KMS
//! - Amazon Web Services KMS
//!
//! Features:
//! - Able to encode using default/current session key (DEK) or receiving it as a parameter
//! - Manual rotation of default/current session key (DEK) or automatic key generation for each of the request
//! - Provides a public and simple implementation for Ring based AEAD encryption without using KMS.
//! - Opt-in for KMS based secure random generator for GCP and AWS instead of Ring.
//!
//! ## Examples:
//!
//! For AWS:
//! ```rust,no_run
//! use kms_aead::providers::AwsKmsProvider;
//! use kms_aead::*;
//! use secret_vault_value::SecretValue;
//!
//!#[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let aws_account_id = config_env_var("ACCOUNT_ID")?;
//!     let aws_key_id: String = config_env_var("KMS_KEY_ID")?;
//!
//!     let kms_ref = kms_aead::providers::AwsKmsKeyRef::new(aws_account_id, aws_key_id);
//!
//!     let encryption: KmsAeadRingEnvelopeEncryption<AwsKmsProvider> =
//!         kms_aead::KmsAeadRingEnvelopeEncryption::new(providers::AwsKmsProvider::new(&kms_ref).await?)
//!             .await?;
//!
//!     let secret_value = SecretValue::from("test-secret");
//!     let test_aad = "test-aad".to_string();
//!
//!     let (encrypted_value, session_key) = encryption.encrypt_value_with_current_key(&test_aad, &secret_value).await?;
//!
//!     let secret_value = encryption
//!         .decrypt_value_with_key(&test_aad, &encrypted_value, &session_key)
//!         .await?;
//!
//!     println!(
//!         "We have our secret back: {}",
//!         secret_value.sensitive_value_to_str().unwrap() == "test-secret"
//!     );
//!
//!     Ok(())
//! }
//!
//! pub fn config_env_var(name: &str) -> Result<String, String> {
//!     std::env::var(name).map_err(|e| format!("{}: {}", name, e))
//! }
//!
//! ```
//!
//! More examples are available at [github](https://github.com/abdolence/kms-aead-rs)
//!

#![allow(unused_parens, clippy::new_without_default)]

extern crate core;

use rvstruct::*;
use secret_vault_value::SecretValue;
pub type KmsAeadResult<T> = std::result::Result<T, errors::KmsAeadError>;

mod api;
pub use api::*;

pub mod errors;

#[cfg(feature = "ring-aead-encryption")]
pub mod ring_encryption;

#[cfg(feature = "ring-aead-encryption")]
mod ring_support;

#[cfg(feature = "ring-aead-encryption")]
pub mod ring_envelope_encryption;

#[cfg(feature = "ring-aead-encryption")]
pub use ring_envelope_encryption::*;

#[derive(Debug, Clone, Eq, PartialEq, ValueStruct)]
pub struct CipherText(pub Vec<u8>);

impl CipherText {
    pub fn to_hex_string(&self) -> String {
        hex::encode(self.value())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, ValueStruct)]
pub struct DataEncryptionKey(pub SecretValue);

#[derive(Debug, Clone, Eq, PartialEq, ValueStruct)]
pub struct EncryptedDataEncryptionKey(pub Vec<u8>);

impl EncryptedDataEncryptionKey {
    pub fn to_hex_string(&self) -> String {
        hex::encode(self.value())
    }
}

pub mod providers;
