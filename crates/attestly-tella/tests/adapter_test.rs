//! End-to-end smoke tests for the attestly-tella adapter scaffold.
//!
//! These tests exercise the public API at the workspace level (rather
//! than the internal unit-test surface inside each module). They are
//! deliberately small in the scaffold; the OTF M1 deliverable expands
//! the test surface to include real ledger interaction + verifier
//! round-trip.

use attestly_tella::{AttestlyReceipt, FieldEvidenceEvent, TellaAdapter, TellaUpload};

fn sample_upload(file_bytes: Vec<u8>, context: Option<&str>) -> TellaUpload {
    let mut b = TellaUpload::builder()
        .file_bytes(file_bytes)
        .source_tool("tella-android")
        .captured_at("2026-06-01T10:00:00Z")
        .metadata(serde_json::json!({"region": "test"}));
    if let Some(c) = context {
        b = b.capture_context(c);
    }
    b.build().expect("upload should build")
}

#[test]
fn end_to_end_smoke_scaffold() {
    struct Stub;
    let adapter = TellaAdapter::new(Stub, Stub);
    let upload = sample_upload(b"evidence-photo-blob".to_vec(), Some("protest-2026"));
    let receipt: AttestlyReceipt = adapter.attest(&upload).expect("attest should succeed");
    assert!(receipt.event_id.starts_with("attestly-tella-"));
    assert_eq!(receipt.upload_hash_hex.len(), 64);
    assert!(receipt.signed_checkpoint_ref.is_none());
}

#[test]
fn event_omits_raw_file_bytes() {
    let upload = sample_upload(b"secret-evidence-content".to_vec(), None);
    let event = FieldEvidenceEvent::from_upload(&upload, "did:web:org.example");
    let serialised = serde_json::to_string(&event).unwrap();
    // The privacy guarantee: raw file bytes never appear in the
    // event (or, by transitivity, on the public transparency log).
    assert!(!serialised.contains("secret-evidence-content"));
}

#[test]
fn receipt_serialises_to_compact_json() {
    struct Stub;
    let adapter = TellaAdapter::new(Stub, Stub);
    let upload = sample_upload(b"data".to_vec(), None);
    let receipt = adapter.attest(&upload).unwrap();
    let s = serde_json::to_string(&receipt).unwrap();
    // Receipt should be small (<2 KB even with reasonable hash sizes)
    // so it can be embedded in a Tella upload's metadata bundle
    // without bloat.
    assert!(s.len() < 2048, "receipt JSON unexpectedly large: {} bytes", s.len());
    assert!(s.contains("event_id"));
    assert!(s.contains("upload_hash_hex"));
}
