//! Signed checkpoint — the public commitment a regulator can verify.
//!
//! The checkpoint contains a Merkle root over all payload hashes plus a
//! signature by the operator-organisation key. Published periodically to
//! a public location (S3, GitHub Pages, IPFS, etc.). Regulators fetch
//! the checkpoint and use it to verify exported evidence bundles.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::{AttestlyError, Result};
use crate::identity::{OrgIdentity, OrgKey};
use crate::merkle::ProofData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCheckpoint {
    /// Wire-format identifier — bump when on-wire structure changes.
    pub version: String,
    /// did:web of the operator org.
    pub origin: String,
    pub tree_size: u64,
    /// Merkle root, hex-encoded.
    pub root_hash: String,
    /// RFC 3339 UTC timestamp.
    pub timestamp: String,
    pub signature: CheckpointSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointSignature {
    pub alg: String,
    pub key_id: String,
    /// Base64url-encoded Ed25519 signature.
    pub value: String,
}

impl SignedCheckpoint {
    pub fn build(org_key: &OrgKey, tree_size: u64, root_hash: [u8; 32]) -> Self {
        let timestamp = OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .expect("rfc3339 format");
        let preimage = canonical_preimage(
            "attestly-checkpoint/v0.1",
            &org_key.identity.did,
            tree_size,
            &root_hash,
            &timestamp,
        );
        let sig = org_key.sign(&preimage);
        Self {
            version: "attestly-checkpoint/v0.1".into(),
            origin: org_key.identity.did.clone(),
            tree_size,
            root_hash: hex::encode(root_hash),
            timestamp,
            signature: CheckpointSignature {
                alg: "Ed25519".into(),
                key_id: org_key.identity.key_id.clone(),
                value: URL_SAFE_NO_PAD.encode(sig.to_bytes()),
            },
        }
    }

    pub fn root_hash_bytes(&self) -> Result<[u8; 32]> {
        let v = hex::decode(&self.root_hash)?;
        v.try_into()
            .map_err(|_| AttestlyError::TamperDetected("root_hash not 32 bytes".into()))
    }

    pub fn verify(&self, identity: &OrgIdentity) -> Result<()> {
        if self.origin != identity.did {
            return Err(AttestlyError::TamperDetected(format!(
                "checkpoint origin {} != identity DID {}",
                self.origin, identity.did
            )));
        }
        let root = self.root_hash_bytes()?;
        let preimage = canonical_preimage(
            &self.version,
            &self.origin,
            self.tree_size,
            &root,
            &self.timestamp,
        );
        let sig_bytes = URL_SAFE_NO_PAD.decode(&self.signature.value)?;
        let arr: [u8; SIGNATURE_LENGTH] = sig_bytes
            .try_into()
            .map_err(|_| AttestlyError::CheckpointSignatureInvalid)?;
        let sig = Signature::from_bytes(&arr);
        identity.verify(&preimage, &sig)
    }
}

/// Stable preimage for signing. Form:
///   version|origin|tree_size|root_hex|timestamp
/// All pipe-delimited, UTF-8.
fn canonical_preimage(
    version: &str,
    origin: &str,
    tree_size: u64,
    root: &[u8; 32],
    timestamp: &str,
) -> Vec<u8> {
    format!(
        "{}|{}|{}|{}|{}",
        version,
        origin,
        tree_size,
        hex::encode(root),
        timestamp
    )
    .into_bytes()
}

/// Inclusion proof JSON file. Lives inside the regulator bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub leaf_seq: i64,
    /// Hex-encoded canonical hash of the leaf.
    pub leaf_hash: String,
    pub proof: ProofData,
    /// Origin DID for the checkpoint that this proof is verified against.
    pub checkpoint_origin: String,
    pub checkpoint_tree_size: u64,
    /// Hex-encoded Merkle root the proof verifies against.
    pub checkpoint_root_hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_signs_and_verifies() {
        let org = OrgKey::generate("did:web:operator.example", "ops-2026-05");
        let cp = SignedCheckpoint::build(&org, 42, [7u8; 32]);
        cp.verify(&org.identity).unwrap();
    }

    #[test]
    fn tampered_root_invalidates_signature() {
        let org = OrgKey::generate("did:web:operator.example", "ops-2026-05");
        let mut cp = SignedCheckpoint::build(&org, 42, [7u8; 32]);
        cp.root_hash = hex::encode([0u8; 32]);
        assert!(cp.verify(&org.identity).is_err());
    }
}
