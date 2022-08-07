use crate::{EncryptedSecretValue, EncryptedSessionKey, KmsAeadResult};
use async_trait::*;
use secret_vault_value::SecretValue;

#[async_trait]
pub trait KmsAeadEncryption<Aad> {

    async fn encrypt_value(
        &self,
        aad: &Aad,
        plain_text: &SecretValue,
        session_key: &SecretValue,
    ) -> KmsAeadResult<EncryptedSecretValue>;

    async fn decrypt_value(
        &self,
        aad: &Aad,
        cipher_text: &EncryptedSecretValue,
        session_key: &SecretValue,
    ) -> KmsAeadResult<SecretValue>;

}

#[async_trait]
pub trait KmsAeadEnvelopeEncryption<Aad> {

    async fn encrypt_value(
        &self,
        aad: &Aad,
        plain_text: &SecretValue,
    ) -> KmsAeadResult<(EncryptedSecretValue, EncryptedSessionKey)>;

    async fn encrypt_value_with_new_session_key(
        &self,
        aad: &Aad,
        plain_text: &SecretValue,
    ) -> KmsAeadResult<(EncryptedSecretValue, EncryptedSessionKey)>;

    async fn decrypt_value(
        &self,
        aad: &Aad,
        encrypted_value: &EncryptedSecretValue,
    ) -> KmsAeadResult<(SecretValue, EncryptedSessionKey)>;

    async fn decrypt_value_with_session_key(
        &self,
        aad: &Aad,
        encrypted_value: &EncryptedSecretValue,
        encrypted_session_key: &EncryptedSessionKey,
    ) -> KmsAeadResult<SecretValue>;

    async fn rotate_session_key(&self)
        -> KmsAeadResult<(EncryptedSessionKey, EncryptedSessionKey)>;
}
