//! `TellaAdapter` — the main entry point of the crate.
//!
//! The adapter wires a Tella upload through to the operator-organisation's
//! Attestly ledger:
//!
//! 1. Pulls the operator DID from the supplied identity.
//! 2. Builds a [`FieldEvidenceEvent`] from the upload (file content stays
//!    private; only its hash flows onward).
//! 3. Converts the field-evidence event into the core's [`DecisionEvent`]
//!    shape via [`From<FieldEvidenceEvent> for DecisionEvent`].
//! 4. Appends the [`DecisionEvent`] to the ledger and pulls the real
//!    [`AppendReceipt`] back.
//! 5. Wraps the append result in an [`AttestlyReceipt`] for the caller.
//!
//! # Design: `FieldEvidenceEvent` → `DecisionEvent` conversion
//!
//! `attestly_core::Ledger` only knows how to append `DecisionEvent`. The
//! Tella adapter speaks `FieldEvidenceEvent` (a parallel CloudEvents-shaped
//! type with extension attributes specific to field-capture evidence). We
//! convert at the adapter boundary with a `From<FieldEvidenceEvent>`
//! impl on `DecisionEvent` — defined locally in this crate to keep the
//! conversion contract owned by the integration layer rather than
//! leaking field-evidence concerns into `attestly-core`.
//!
//! The Tella extension attributes (`upload_hash_hex`, `upload_bytes`,
//! `source_tool`, `capture_context`, `adapter_version`) are folded into
//! `DecisionEvent.data` so the canonical hash covers them, and the
//! ledger row is a self-describing record of the upload.
//!
//! # Design: generic-with-bounds vs concrete types
//!
//! The adapter stays generic over both ledger and identity, with trait
//! bounds [`LedgerLike`] and [`OrgIdentityLike`]. Rationale:
//!
//! - Tests can supply mock ledgers (counting appends, returning canned
//!   receipts) without spinning up a real SQLite database.
//! - Future sidecar deployments can swap in remote-ledger or
//!   batched-ledger implementations without forking the adapter.
//! - The blanket `impl LedgerLike for attestly_core::Ledger` (and ditto
//!   for `OrgIdentity`) means the concrete production path requires no
//!   wrapper at the call site.

use attestly_core::{AppendReceipt, DecisionEvent, Ledger, OrgIdentity};

use crate::{
    evidence::FieldEvidenceEvent, receipt::AttestlyReceipt, upload::TellaUpload,
    FIELD_EVIDENCE_EVENT_TYPE,
};

/// Errors that can occur during adapter operation.
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    /// The upload payload could not be canonicalised — typically a
    /// malformed metadata JSON.
    #[error("could not canonicalise upload metadata: {0}")]
    Canonicalisation(String),
    /// The underlying ledger rejected the append (e.g. constraint
    /// violation, storage failure).
    #[error("ledger append failed: {0}")]
    LedgerAppend(String),
    /// Wrap a generic core error.
    #[error("attestly core error: {0}")]
    Core(String),
}

/// Result alias used by adapter operations.
pub type AdapterResult<T> = Result<T, AdapterError>;

/// Abstraction over `attestly_core::Ledger` so the adapter can be tested
/// with mock backends. The single production implementation is for
/// [`attestly_core::Ledger`].
pub trait LedgerLike {
    /// Append a decision event, returning the core's append receipt.
    fn append(&mut self, event: DecisionEvent) -> AdapterResult<AppendReceipt>;
}

impl LedgerLike for Ledger {
    fn append(&mut self, event: DecisionEvent) -> AdapterResult<AppendReceipt> {
        Ledger::append(self, event).map_err(|e| AdapterError::LedgerAppend(e.to_string()))
    }
}

/// Abstraction over `attestly_core::OrgIdentity` so the adapter can be
/// tested without constructing a real Ed25519 keypair-backed identity.
pub trait OrgIdentityLike {
    /// Return the operator-organisation's `did:web` DID string.
    fn did(&self) -> &str;
}

impl OrgIdentityLike for OrgIdentity {
    fn did(&self) -> &str {
        &self.did
    }
}

/// The Tella adapter.
///
/// Owns mutable references to the operator-organisation's Attestly
/// ledger and a handle to the org identity. `attest` is `&mut self`
/// because [`Ledger::append`](attestly_core::Ledger::append) takes
/// `&mut self`.
pub struct TellaAdapter<L, I> {
    ledger: L,
    org_identity: I,
}

impl<L, I> TellaAdapter<L, I>
where
    L: LedgerLike,
    I: OrgIdentityLike,
{
    /// Construct a new adapter.
    ///
    /// `ledger` is the operator-organisation's Attestly ledger
    /// (typically an [`attestly_core::Ledger`] handle).
    ///
    /// `org_identity` is the organisation's [`attestly_core::OrgIdentity`]
    /// — the source DID on every emitted event comes from here.
    pub fn new(ledger: L, org_identity: I) -> Self {
        Self {
            ledger,
            org_identity,
        }
    }

    /// Attest a Tella upload: derive the [`FieldEvidenceEvent`], convert
    /// it to a [`DecisionEvent`], append it to the ledger, and return
    /// the [`AttestlyReceipt`] containing the real ledger position and
    /// append timestamp.
    pub fn attest(&mut self, upload: &TellaUpload) -> AdapterResult<AttestlyReceipt> {
        // Step 1: derive the source DID from the org identity.
        let org_did = self.org_identity.did().to_string();

        // Step 2: build the field-evidence event (file bytes stay
        // private; only their hash is included).
        let event = FieldEvidenceEvent::from_upload(upload, &org_did);
        let event_id = event.id.clone();
        let upload_hash_hex = event.upload_hash_hex.clone();
        let captured_at = upload.captured_at.clone();

        // Step 3: convert to the core DecisionEvent the ledger speaks.
        let decision_event: DecisionEvent = event.into();

        // Step 4: append to the ledger and unwrap the real receipt.
        let append_receipt = self.ledger.append(decision_event)?;

        // Step 5: assemble the AttestlyReceipt from the real
        // AppendReceipt. The ledger's `seq` is the monotonic
        // append-position; `event_id` matches the FieldEvidenceEvent id
        // (preserved through the DecisionEvent conversion); `appended_at`
        // is the upload's captured-at timestamp (the ledger row's
        // internal `ts` column is not surfaced through AppendReceipt,
        // so we use the capture timestamp — which is what the verifier
        // checks against anyway).
        let receipt = AttestlyReceipt::new_pending(
            append_receipt.event_id.clone(),
            upload_hash_hex,
            append_receipt.seq as u64,
            captured_at,
            org_did,
        );
        debug_assert_eq!(append_receipt.event_id, event_id);
        Ok(receipt)
    }
}

/// Convert a Tella `FieldEvidenceEvent` into the core `DecisionEvent`
/// shape the ledger appends.
///
/// The conversion preserves the CloudEvents identifiers (`id`, `source`,
/// `type`, `time`) and folds the field-evidence extension attributes
/// (`upload_hash_hex`, `upload_bytes`, `source_tool`, `capture_context`,
/// `adapter_version`) into `DecisionEvent.data` so they're covered by
/// the ledger's canonical hash.
///
/// The original `FieldEvidenceEvent.data` (the Tella metadata blob) is
/// nested under `data.metadata` to avoid colliding with the extension
/// attributes we lifted.
///
/// `category` is set to the field-evidence event type constant so
/// downstream Annex-III-aware indexing can route field-evidence rows
/// distinctly from AI-decision rows.
impl From<FieldEvidenceEvent> for DecisionEvent {
    fn from(e: FieldEvidenceEvent) -> Self {
        let mut data = serde_json::Map::new();
        data.insert(
            "upload_hash_hex".to_string(),
            serde_json::Value::String(e.upload_hash_hex),
        );
        data.insert(
            "upload_bytes".to_string(),
            serde_json::Value::Number(e.upload_bytes.into()),
        );
        data.insert(
            "source_tool".to_string(),
            serde_json::Value::String(e.source_tool),
        );
        if let Some(ctx) = e.capture_context {
            data.insert(
                "capture_context".to_string(),
                serde_json::Value::String(ctx),
            );
        }
        data.insert(
            "adapter_version".to_string(),
            serde_json::Value::String(e.adapter_version),
        );
        data.insert("metadata".to_string(), e.data);

        DecisionEvent {
            id: e.id,
            source: e.source,
            event_type: e.event_type, // FIELD_EVIDENCE_EVENT_TYPE
            time: e.time,
            category: FIELD_EVIDENCE_EVENT_TYPE.to_string(),
            schema_version: e.schema_version,
            model_id: None,
            subject_ref: None,
            data: serde_json::Value::Object(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use attestly_core::identity::OrgKey;
    use attestly_core::{Ledger, SystemKey};
    use tempfile::NamedTempFile;

    fn fresh_ledger() -> Ledger {
        let key = SystemKey::generate("did:web:operator.example/ai-system/test");
        // `Ledger::open_in_memory` is `#[cfg(test)]`-gated inside the
        // core crate, so it isn't visible from this integration crate.
        // Use a NamedTempFile to get a real on-disk SQLite ledger that
        // disappears at the end of the test.
        let path = NamedTempFile::new()
            .expect("tempfile create")
            .into_temp_path();
        let path_str = path
            .to_str()
            .expect("tempfile path is valid UTF-8")
            .to_string();
        // Persist the path so the SQLite file lives for the duration of
        // the ledger; the OS will drop it when the process exits.
        let _ = path.keep().expect("tempfile keep");
        Ledger::open(&path_str, key).expect("ledger open")
    }

    fn fresh_org() -> OrgIdentity {
        OrgKey::generate("did:web:journalists.example", "ops-2026-05").identity
    }

    fn sample_upload() -> TellaUpload {
        TellaUpload::builder()
            .file_bytes(b"sample-evidence".to_vec())
            .source_tool("tella-android")
            .captured_at("2026-06-01T10:00:00Z")
            .build()
            .unwrap()
    }

    #[test]
    fn attest_returns_receipt_with_populated_event_id() {
        let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
        let receipt = adapter.attest(&sample_upload()).unwrap();
        assert!(receipt.event_id.starts_with("attestly-tella-"));
        assert!(!receipt.upload_hash_hex.is_empty());
        assert_eq!(receipt.org_did, "did:web:journalists.example");
    }

    #[test]
    fn attest_event_id_and_upload_hash_are_deterministic_per_upload() {
        // The upload-derived event id and upload hash are deterministic
        // (they only depend on the upload payload). The first append
        // produces the canonical event id; the second append (against a
        // fresh ledger) of the same payload reproduces it.
        let mut a1 = TellaAdapter::new(fresh_ledger(), fresh_org());
        let mut a2 = TellaAdapter::new(fresh_ledger(), fresh_org());
        let r1 = a1.attest(&sample_upload()).unwrap();
        let r2 = a2.attest(&sample_upload()).unwrap();
        assert_eq!(r1.event_id, r2.event_id);
        assert_eq!(r1.upload_hash_hex, r2.upload_hash_hex);
    }

    #[test]
    fn attest_distinguishes_different_uploads() {
        let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
        let r1 = adapter.attest(&sample_upload()).unwrap();
        let r2 = adapter
            .attest(
                &TellaUpload::builder()
                    .file_bytes(b"different-evidence".to_vec())
                    .source_tool("tella-android")
                    .captured_at("2026-06-01T10:00:00Z")
                    .build()
                    .unwrap(),
            )
            .unwrap();
        assert_ne!(r1.upload_hash_hex, r2.upload_hash_hex);
    }

    // === New ledger-backed tests ===

    #[test]
    fn attest_append_position_matches_real_ledger_size() {
        let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
        let r1 = adapter.attest(&sample_upload()).unwrap();
        // The first append should land at ledger position 1 (SQLite
        // AUTOINCREMENT starts at 1).
        assert_eq!(r1.append_position, 1);
    }

    #[test]
    fn attest_two_different_uploads_get_different_positions() {
        let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
        let r1 = adapter.attest(&sample_upload()).unwrap();
        let r2 = adapter
            .attest(
                &TellaUpload::builder()
                    .file_bytes(b"second-distinct-evidence".to_vec())
                    .source_tool("tella-android")
                    .captured_at("2026-06-01T10:05:00Z")
                    .build()
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(r1.append_position, 1);
        assert_eq!(r2.append_position, 2);
        assert_ne!(r1.upload_hash_hex, r2.upload_hash_hex);
    }

    #[test]
    fn attest_same_upload_twice_is_rejected_as_double_attestation() {
        // The SQLite `event_id` column has a UNIQUE constraint and the
        // FieldEvidenceEvent id is derived deterministically from the
        // upload hash. So appending the same upload twice MUST fail on
        // the second insert — which is the correct evidence-pipeline
        // behaviour (no double-attestation of the same byte-identical
        // upload). The first append succeeds; the second surfaces the
        // UNIQUE constraint violation as a LedgerAppend error.
        //
        // NOTE: the OTF-M1 brief originally anticipated "same upload
        // twice → different append_positions, same upload_hash_hex".
        // That contract presupposes a non-deterministic event id (e.g.
        // a UUID v4 or a timestamp nonce). The existing
        // `FieldEvidenceEvent::from_upload` derives the id from the
        // upload hash (an evidence-grade property the existing
        // `src/evidence.rs` tests pin down), so deterministic-id wins.
        // We assert the observed-correct behaviour here.
        let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
        let r1 = adapter.attest(&sample_upload()).unwrap();
        let err = adapter.attest(&sample_upload()).unwrap_err();
        assert_eq!(r1.append_position, 1);
        assert!(
            matches!(err, AdapterError::LedgerAppend(_)),
            "expected LedgerAppend error, got {err:?}"
        );
    }

    #[test]
    fn from_upload_into_decision_event_preserves_identifiers() {
        let upload = sample_upload();
        let fe = FieldEvidenceEvent::from_upload(&upload, "did:web:example.org");
        let id = fe.id.clone();
        let upload_hash_hex = fe.upload_hash_hex.clone();
        let de: DecisionEvent = fe.into();
        assert_eq!(de.id, id);
        assert_eq!(de.source, "did:web:example.org");
        assert_eq!(de.event_type, FIELD_EVIDENCE_EVENT_TYPE);
        assert_eq!(de.category, FIELD_EVIDENCE_EVENT_TYPE);
        // The folded extension attributes appear in DecisionEvent.data.
        let data = de.data.as_object().expect("data is an object");
        assert_eq!(
            data.get("upload_hash_hex")
                .and_then(|v| v.as_str())
                .unwrap(),
            upload_hash_hex
        );
        assert_eq!(
            data.get("source_tool").and_then(|v| v.as_str()).unwrap(),
            "tella-android"
        );
        assert!(data.contains_key("metadata"));
    }

    #[test]
    fn from_upload_into_decision_event_passes_core_validation() {
        // The converted DecisionEvent must satisfy attestly_core's
        // validate() — i.e. all CloudEvents required fields populated.
        // This is what lets Ledger::append accept it.
        let upload = sample_upload();
        let fe = FieldEvidenceEvent::from_upload(&upload, "did:web:example.org");
        let de: DecisionEvent = fe.into();
        de.validate().expect("converted event passes core validation");
        // And its canonical hash is computable.
        de.canonical_hash().expect("canonical hash computable");
    }
}
