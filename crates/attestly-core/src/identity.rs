//! Cryptographic identity for AI systems and operator organisations.
//!
//! - `SystemKey` is an Ed25519 keypair held by an AI system instance.
//!   It signs every decision event before append.
//! - `OrgIdentity` is the operator-organisation `did:web` identity that
//!   signs Merkle-rooted checkpoints. Its public key is published at
//!   `https://<domain>/.well-known/did.json` for regulator lookup.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::error::{AttestlyError, Result};

/// Ed25519 keypair for an AI system instance.
pub struct SystemKey {
    signing: SigningKey,
    /// Canonical DID for this AI system, e.g. `did:web:operator.example/ai-system/credit-scorer`.
    pub did: String,
}

impl SystemKey {
    pub fn generate(did: impl Into<String>) -> Self {
        Self {
            signing: SigningKey::generate(&mut OsRng),
            did: did.into(),
        }
    }

    pub fn from_seed(did: impl Into<String>, seed: [u8; SECRET_KEY_LENGTH]) -> Self {
        Self {
            signing: SigningKey::from_bytes(&seed),
            did: did.into(),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing.sign(msg)
    }

    pub fn public_key_b64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.verifying_key().to_bytes())
    }

    pub fn seed_b64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.signing.to_bytes())
    }
}

/// Operator organisation identity. Uses `did:web` resolution semantics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgIdentity {
    /// e.g. `did:web:operator.example`
    pub did: String,
    /// e.g. `did:web:operator.example#ops-2026-05`
    pub key_id: String,
    /// Public verifying key, base64url-encoded (32 bytes Ed25519).
    pub verifying_key_b64: String,
}

impl OrgIdentity {
    pub fn new(did: impl Into<String>, key_id: impl Into<String>, vk: &VerifyingKey) -> Self {
        Self {
            did: did.into(),
            key_id: key_id.into(),
            verifying_key_b64: URL_SAFE_NO_PAD.encode(vk.to_bytes()),
        }
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey> {
        let bytes = URL_SAFE_NO_PAD.decode(&self.verifying_key_b64)?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| AttestlyError::InvalidDid("public key must be 32 bytes".into()))?;
        Ok(VerifyingKey::from_bytes(&arr)?)
    }

    pub fn verify(&self, msg: &[u8], sig: &Signature) -> Result<()> {
        self.verifying_key()?
            .verify(msg, sig)
            .map_err(|_| AttestlyError::CheckpointSignatureInvalid)
    }

    /// Build a minimal did:web DID Document suitable for serving at
    /// `/.well-known/did.json`.
    pub fn did_document(&self) -> serde_json::Value {
        serde_json::json!({
            "@context": ["https://www.w3.org/ns/did/v1"],
            "id": self.did,
            "verificationMethod": [{
                "id": self.key_id,
                "type": "Ed25519VerificationKey2020",
                "controller": self.did,
                "publicKeyMultibase": format!("z{}", self.verifying_key_b64),
            }],
            "assertionMethod": [self.key_id],
        })
    }
}

/// An operator-org keypair (Ed25519). Used to sign checkpoints.
pub struct OrgKey {
    signing: SigningKey,
    pub identity: OrgIdentity,
}

impl OrgKey {
    pub fn generate(did: impl Into<String>, key_id_suffix: &str) -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        let did = did.into();
        let identity = OrgIdentity::new(
            did.clone(),
            format!("{did}#{key_id_suffix}"),
            &signing.verifying_key(),
        );
        Self { signing, identity }
    }

    pub fn from_seed(
        did: impl Into<String>,
        key_id_suffix: &str,
        seed: [u8; SECRET_KEY_LENGTH],
    ) -> Self {
        let signing = SigningKey::from_bytes(&seed);
        let did = did.into();
        let identity = OrgIdentity::new(
            did.clone(),
            format!("{did}#{key_id_suffix}"),
            &signing.verifying_key(),
        );
        Self { signing, identity }
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing.sign(msg)
    }

    pub fn seed_b64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.signing.to_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_key_signs_and_verifies() {
        let key = SystemKey::generate("did:web:operator.example/ai-system/test");
        let msg = b"a credit decision";
        let sig = key.sign(msg);
        assert!(key.verifying_key().verify(msg, &sig).is_ok());
    }

    #[test]
    fn org_identity_roundtrip() {
        let org_key = OrgKey::generate("did:web:operator.example", "ops-2026-05");
        let msg = b"a checkpoint";
        let sig = org_key.sign(msg);
        org_key.identity.verify(msg, &sig).unwrap();
    }

    #[test]
    fn did_document_has_required_fields() {
        let org_key = OrgKey::generate("did:web:operator.example", "ops-2026-05");
        let doc = org_key.identity.did_document();
        assert_eq!(doc["id"], "did:web:operator.example");
        assert_eq!(
            doc["verificationMethod"][0]["controller"],
            "did:web:operator.example"
        );
    }
}
