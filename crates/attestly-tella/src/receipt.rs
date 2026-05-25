//! `AttestlyReceipt` — the proof returned to Tella's destination backend
//! after a successful adapter attestation.
//!
//! The receipt is what the verifier consumes when a future
//! prosecutor/lawyer/regulator drops the Tella-captured evidence into
//! the Attestly browser verifier. It contains everything needed to
//! verify the upload was attested at the claimed time, was not
//! subsequently altered, and was committed to the public transparency
//! log under the claimed organisation's identity.

use serde::{Deserialize, Serialize};

use crate::ADAPTER_SCHEMA_VERSION;

/// Receipt returned by [`crate::TellaAdapter::attest`].
///
/// The receipt should be stored alongside the captured evidence
/// (typically inside the destination Uwazi/ODK/Nextcloud record) so
/// that the verification chain is preserved even if the original
/// Attestly ledger row is later disputed or unavailable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestlyReceipt {
    /// CloudEvents `id` of the appended [`crate::FieldEvidenceEvent`].
    pub event_id: String,
    /// Hex-encoded SHA-256 hash of the canonical upload payload.
    pub upload_hash_hex: String,
    /// Monotonically-increasing position of the event in the
    /// organisation's Attestly ledger.
    pub append_position: u64,
    /// RFC 3339 timestamp recorded at append time.
    pub appended_at: String,
    /// DID of the capturing organisation (matches the event `source`).
    pub org_did: String,
    /// Reference to the Signed Tree Head that will cover (or already
    /// covers) this event.
    ///
    /// `None` if the receipt is returned before the next checkpoint is
    /// published. A future receipt update can fill this field in once
    /// the checkpoint is available; the verifier handles both cases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_checkpoint_ref: Option<SignedCheckpointRef>,
    /// Adapter-specific schema version of this receipt.
    pub adapter_version: String,
    /// Attestly Decision Schema version of the underlying event.
    pub schema_version: String,
}

impl AttestlyReceipt {
    /// Construct a new receipt referring to a freshly-appended event,
    /// without yet a published checkpoint covering it.
    pub fn new_pending(
        event_id: impl Into<String>,
        upload_hash_hex: impl Into<String>,
        append_position: u64,
        appended_at: impl Into<String>,
        org_did: impl Into<String>,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            upload_hash_hex: upload_hash_hex.into(),
            append_position,
            appended_at: appended_at.into(),
            org_did: org_did.into(),
            signed_checkpoint_ref: None,
            adapter_version: ADAPTER_SCHEMA_VERSION.to_string(),
            schema_version: "0.1".to_string(),
        }
    }

    /// Set the signed-checkpoint reference (called after the next
    /// checkpoint is published).
    pub fn with_checkpoint(mut self, c: SignedCheckpointRef) -> Self {
        self.signed_checkpoint_ref = Some(c);
        self
    }
}

/// A pointer to the Signed Tree Head that covers an event.
///
/// The verifier uses this reference to fetch the full checkpoint plus
/// the inclusion proof for the event and verify both against the
/// organisation's published DID document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCheckpointRef {
    /// Tree size of the Signed Tree Head.
    pub tree_size: u64,
    /// Hex-encoded Merkle root.
    pub root_hash_hex: String,
    /// RFC 3339 timestamp on the checkpoint.
    pub checkpoint_time: String,
    /// HTTPS URL where the full signed checkpoint can be fetched.
    pub checkpoint_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pending_constructs_well_formed_receipt() {
        let r = AttestlyReceipt::new_pending(
            "attestly-tella-abc123",
            "deadbeef".repeat(8),
            42,
            "2026-06-01T10:23:11Z",
            "did:web:example.org",
        );
        assert_eq!(r.event_id, "attestly-tella-abc123");
        assert_eq!(r.append_position, 42);
        assert!(r.signed_checkpoint_ref.is_none());
        assert_eq!(r.adapter_version, ADAPTER_SCHEMA_VERSION);
    }

    #[test]
    fn with_checkpoint_attaches_reference() {
        let r = AttestlyReceipt::new_pending(
            "id",
            "hash",
            1,
            "t",
            "did",
        )
        .with_checkpoint(SignedCheckpointRef {
            tree_size: 100,
            root_hash_hex: "abc".to_string(),
            checkpoint_time: "2026-06-01T11:00:00Z".to_string(),
            checkpoint_url: "https://logs.attestly.org/sth/100".to_string(),
        });
        assert!(r.signed_checkpoint_ref.is_some());
        assert_eq!(r.signed_checkpoint_ref.unwrap().tree_size, 100);
    }

    #[test]
    fn receipt_round_trip_serialisation() {
        let r = AttestlyReceipt::new_pending(
            "id",
            "hash",
            1,
            "t",
            "did:web:org.example",
        );
        let s = serde_json::to_string(&r).unwrap();
        let back: AttestlyReceipt = serde_json::from_str(&s).unwrap();
        assert_eq!(back.org_did, "did:web:org.example");
    }
}
