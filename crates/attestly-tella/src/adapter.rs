//! `TellaAdapter` — the main entry point of the crate.
//!
//! The adapter is intentionally minimal in this scaffold. The OTF M1
//! deliverable will flesh it out with:
//!
//! - Real wiring into [`attestly_core::Ledger`] (currently the scaffold
//!   defines the API contract but does not call into the core ledger).
//! - Async variants for high-throughput deployments.
//! - Optional batching to reduce per-upload ledger overhead.
//! - Sidecar-binary wrapping for deployments that prefer a separate
//!   process rather than embedding the library.

use crate::{
    evidence::FieldEvidenceEvent,
    receipt::AttestlyReceipt,
    upload::TellaUpload,
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

/// The Tella adapter.
///
/// In this scaffold the adapter holds opaque handles to the underlying
/// attestly-core ledger and the operator-organisation identity. The
/// concrete generic types will be filled in under OTF M1 once the
/// real ledger and identity APIs are wired up.
#[allow(dead_code)] // fields are placeholders wired up in OTF M1
pub struct TellaAdapter<L, I> {
    ledger: L,
    org_identity: I,
}

impl<L, I> TellaAdapter<L, I> {
    /// Construct a new adapter.
    ///
    /// `ledger` is the operator-organisation's Attestly ledger
    /// (typically an [`attestly_core::Ledger`] handle).
    ///
    /// `org_identity` is the organisation's [`attestly_core::OrgIdentity`]
    /// — used to derive the `did:web` source field on emitted events.
    pub fn new(ledger: L, org_identity: I) -> Self {
        Self {
            ledger,
            org_identity,
        }
    }

    /// Attest a Tella upload: derive the [`FieldEvidenceEvent`],
    /// append it to the ledger, return the [`AttestlyReceipt`].
    ///
    /// **Scaffold note**: in this v0.0 the actual ledger append is
    /// stubbed (returns a synthetic `AttestlyReceipt`). The full
    /// implementation under OTF M1 will:
    ///
    /// 1. Call `self.org_identity.did_web()` to derive the source DID.
    /// 2. Construct the [`FieldEvidenceEvent`] via
    ///    [`FieldEvidenceEvent::from_upload`].
    /// 3. Call `self.ledger.append(event)` to obtain a real
    ///    `AppendReceipt`.
    /// 4. Convert the core receipt into an [`AttestlyReceipt`] and
    ///    return it.
    pub fn attest(&self, upload: &TellaUpload) -> AdapterResult<AttestlyReceipt> {
        // Step 1: build the field-evidence event using a placeholder DID.
        // The full implementation pulls the DID from `self.org_identity`.
        let placeholder_did = "did:web:scaffold.attestly.invalid";
        let event = FieldEvidenceEvent::from_upload(upload, placeholder_did);

        // Step 2: STUB — would call `self.ledger.append(event)` here.
        // For the scaffold we synthesize a deterministic receipt so the
        // public API can be exercised in tests.
        let receipt = AttestlyReceipt::new_pending(
            event.id.clone(),
            event.upload_hash_hex.clone(),
            0,
            upload.captured_at.clone(),
            event.source.clone(),
        );
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A stub ledger and identity for scaffold-time API exercise. The
    // full implementation replaces these with real attestly-core types.
    struct StubLedger;
    struct StubIdentity;

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
        let adapter = TellaAdapter::new(StubLedger, StubIdentity);
        let receipt = adapter.attest(&sample_upload()).unwrap();
        assert!(receipt.event_id.starts_with("attestly-tella-"));
        assert!(!receipt.upload_hash_hex.is_empty());
        assert_eq!(receipt.org_did, "did:web:scaffold.attestly.invalid");
    }

    #[test]
    fn attest_is_deterministic_for_identical_uploads() {
        let adapter = TellaAdapter::new(StubLedger, StubIdentity);
        let r1 = adapter.attest(&sample_upload()).unwrap();
        let r2 = adapter.attest(&sample_upload()).unwrap();
        assert_eq!(r1.event_id, r2.event_id);
        assert_eq!(r1.upload_hash_hex, r2.upload_hash_hex);
    }

    #[test]
    fn attest_distinguishes_different_uploads() {
        let adapter = TellaAdapter::new(StubLedger, StubIdentity);
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
}
