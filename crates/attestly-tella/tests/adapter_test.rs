//! End-to-end smoke tests for the attestly-tella adapter.
//!
//! These tests exercise the public API at the workspace level (rather
//! than the internal unit-test surface inside each module). They use
//! the real `attestly_core::Ledger` + `OrgIdentity` so the adapter is
//! exercised through its production wiring path.

use attestly_core::identity::OrgKey;
use attestly_core::{Ledger, SystemKey};
use attestly_tella::{AttestlyReceipt, FieldEvidenceEvent, TellaAdapter, TellaUpload};
use tempfile::NamedTempFile;

fn fresh_ledger() -> Ledger {
    let key = SystemKey::generate("did:web:operator.example/ai-system/test");
    let path = NamedTempFile::new()
        .expect("tempfile create")
        .into_temp_path();
    let path_str = path
        .to_str()
        .expect("tempfile path is valid UTF-8")
        .to_string();
    let _ = path.keep().expect("tempfile keep");
    Ledger::open(&path_str, key).expect("ledger open")
}

fn fresh_org() -> attestly_core::OrgIdentity {
    OrgKey::generate("did:web:journalists.example", "ops-2026-05").identity
}

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
fn end_to_end_smoke() {
    let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
    let upload = sample_upload(b"evidence-photo-blob".to_vec(), Some("protest-2026"));
    let receipt: AttestlyReceipt = adapter.attest(&upload).expect("attest should succeed");
    assert!(receipt.event_id.starts_with("attestly-tella-"));
    assert_eq!(receipt.upload_hash_hex.len(), 64);
    assert!(receipt.signed_checkpoint_ref.is_none());
    assert_eq!(receipt.append_position, 1);
    assert_eq!(receipt.org_did, "did:web:journalists.example");
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
    let mut adapter = TellaAdapter::new(fresh_ledger(), fresh_org());
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
