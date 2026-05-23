//! Attestly Verifier — standalone bundle verification with no database dependency.
//!
//! What a regulator runs locally to verify an evidence bundle exported by an
//! operator. Reads only JSON + ZIP; no Postgres, no SQLite, no operator
//! infrastructure. Exit codes from the CLI surface map directly to
//! [`VerifyResult`] variants.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use attestly_core::checkpoint::{InclusionProof, SignedCheckpoint};
use attestly_core::event::DecisionEvent;
use attestly_core::identity::OrgIdentity;
use attestly_core::merkle;

#[derive(Debug)]
pub enum VerifyResult {
    Valid {
        decision_seq: i64,
        checkpoint_origin: String,
        checkpoint_tree_size: u64,
    },
    Tampered {
        reason: String,
        expected: String,
        actual: String,
    },
    Malformed {
        reason: String,
    },
}

impl VerifyResult {
    pub fn exit_code(&self) -> i32 {
        match self {
            VerifyResult::Valid { .. } => 0,
            VerifyResult::Tampered { .. } => 1,
            VerifyResult::Malformed { .. } => 2,
        }
    }
}

/// Verify an evidence bundle (a ZIP produced by `attestly export`).
///
/// Bundle layout:
/// - `decision.json`         — canonical DecisionEvent
/// - `inclusion-proof.json`  — Merkle inclusion proof for the decision
/// - `checkpoint.json`       — SignedCheckpoint the proof verifies against
/// - `did-document.json`     — operator's did:web DID Document (pubkey)
/// - `README.md`             — human-readable verification instructions
pub fn verify_bundle(bundle_path: &Path) -> VerifyResult {
    let file = match File::open(bundle_path) {
        Ok(f) => f,
        Err(e) => {
            return VerifyResult::Malformed {
                reason: format!("cannot open bundle: {e}"),
            }
        }
    };
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            return VerifyResult::Malformed {
                reason: format!("invalid zip: {e}"),
            }
        }
    };

    let decision: DecisionEvent = match read_json(&mut archive, "decision.json") {
        Ok(v) => v,
        Err(e) => return VerifyResult::Malformed { reason: e },
    };
    let proof: InclusionProof = match read_json(&mut archive, "inclusion-proof.json") {
        Ok(v) => v,
        Err(e) => return VerifyResult::Malformed { reason: e },
    };
    let checkpoint: SignedCheckpoint = match read_json(&mut archive, "checkpoint.json") {
        Ok(v) => v,
        Err(e) => return VerifyResult::Malformed { reason: e },
    };
    let did_doc: serde_json::Value = match read_json(&mut archive, "did-document.json") {
        Ok(v) => v,
        Err(e) => return VerifyResult::Malformed { reason: e },
    };

    // 1. Cross-check: the proof's checkpoint reference matches the actual checkpoint.
    if proof.checkpoint_origin != checkpoint.origin
        || proof.checkpoint_tree_size != checkpoint.tree_size
        || proof.checkpoint_root_hash != checkpoint.root_hash
    {
        return VerifyResult::Tampered {
            reason: "inclusion proof references a different checkpoint than the one in the bundle"
                .into(),
            expected: format!(
                "origin={}, tree_size={}, root={}",
                checkpoint.origin, checkpoint.tree_size, checkpoint.root_hash
            ),
            actual: format!(
                "origin={}, tree_size={}, root={}",
                proof.checkpoint_origin, proof.checkpoint_tree_size, proof.checkpoint_root_hash
            ),
        };
    }

    // 2. Extract operator identity from DID Document.
    let identity = match extract_identity(&did_doc, &checkpoint.origin) {
        Ok(id) => id,
        Err(e) => return VerifyResult::Malformed { reason: e },
    };

    // 3. Verify the checkpoint's Ed25519 signature.
    if let Err(e) = checkpoint.verify(&identity) {
        return VerifyResult::Tampered {
            reason: format!("checkpoint signature invalid: {e}"),
            expected: format!("signature by {}", identity.key_id),
            actual: "no valid signature".into(),
        };
    }

    // 4. Recompute the canonical hash of the decision payload.
    let canonical = match decision.canonical_hash() {
        Ok(h) => h,
        Err(e) => {
            return VerifyResult::Malformed {
                reason: e.to_string(),
            }
        }
    };
    let canonical_hex = hex::encode(canonical);
    if canonical_hex != proof.leaf_hash {
        return VerifyResult::Tampered {
            reason: format!(
                "decision payload no longer matches the leaf hash recorded in the inclusion proof (seq={})",
                proof.leaf_seq
            ),
            expected: format!("leaf_hash={}", proof.leaf_hash),
            actual: format!("recomputed canonical hash={canonical_hex}"),
        };
    }

    // 5. Verify the Merkle inclusion proof.
    let root: [u8; 32] = match hex::decode(&checkpoint.root_hash)
        .ok()
        .and_then(|v| v.try_into().ok())
    {
        Some(arr) => arr,
        None => {
            return VerifyResult::Malformed {
                reason: "checkpoint root_hash is not 32-byte hex".into(),
            }
        }
    };
    if !merkle::verify_inclusion(&proof.proof, canonical, root) {
        return VerifyResult::Tampered {
            reason: format!(
                "Merkle inclusion proof for seq={} does not validate against checkpoint root",
                proof.leaf_seq
            ),
            expected: format!("root={}", checkpoint.root_hash),
            actual: "proof did not reconstruct expected root".into(),
        };
    }

    VerifyResult::Valid {
        decision_seq: proof.leaf_seq,
        checkpoint_origin: checkpoint.origin,
        checkpoint_tree_size: checkpoint.tree_size,
    }
}

fn read_json<T, R>(archive: &mut zip::ZipArchive<R>, name: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
    R: Read + std::io::Seek,
{
    let mut file = archive
        .by_name(name)
        .map_err(|e| format!("bundle missing {name}: {e}"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| format!("cannot read {name}: {e}"))?;
    serde_json::from_str(&buf).map_err(|e| format!("invalid {name}: {e}"))
}

/// Extract the operator's verifying key from the did:web DID Document and
/// build an [`OrgIdentity`] suitable for checkpoint verification.
fn extract_identity(
    did_doc: &serde_json::Value,
    expected_did: &str,
) -> Result<OrgIdentity, String> {
    let did = did_doc
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "DID document missing 'id'".to_string())?;
    if did != expected_did {
        return Err(format!(
            "DID document id {did} does not match checkpoint origin {expected_did}"
        ));
    }
    let methods = did_doc
        .get("verificationMethod")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "DID document missing 'verificationMethod' array".to_string())?;
    let first = methods
        .first()
        .ok_or_else(|| "DID document has empty verificationMethod array".to_string())?;
    let key_id = first
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "verificationMethod[0] missing 'id'".to_string())?;
    let multibase = first
        .get("publicKeyMultibase")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "verificationMethod[0] missing 'publicKeyMultibase'".to_string())?;
    // Strip the multibase 'z' prefix to get base64url-encoded key bytes.
    let stripped = multibase
        .strip_prefix('z')
        .ok_or_else(|| "publicKeyMultibase must start with 'z'".to_string())?;
    Ok(OrgIdentity {
        did: did.to_string(),
        key_id: key_id.to_string(),
        verifying_key_b64: stripped.to_string(),
    })
}
