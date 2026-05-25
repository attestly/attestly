//! `FieldEvidenceEvent` — the Attestly event type for Tella uploads.
//!
//! Parallel to `DecisionEvent` in `attestly-core` (which is for AI
//! system decisions under EU AI Act Article 12), `FieldEvidenceEvent`
//! is the event type for civil-society field-capture evidence under
//! the broader Attestly Decision Schema.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{TellaUpload, ADAPTER_SCHEMA_VERSION, FIELD_EVIDENCE_EVENT_TYPE};

/// An Attestly-ledger event derived from a Tella upload.
///
/// The shape is CloudEvents v1.0-compatible with Attestly extension
/// attributes specific to field evidence (as opposed to AI system
/// decisions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldEvidenceEvent {
    /// CloudEvents `id` — globally unique per event.
    pub id: String,
    /// CloudEvents `source` — the capturing organisation's DID.
    pub source: String,
    /// CloudEvents `type` — always [`FIELD_EVIDENCE_EVENT_TYPE`] for
    /// Tella-adapter events.
    #[serde(rename = "type")]
    pub event_type: String,
    /// CloudEvents `time` — RFC 3339, equal to the capture timestamp.
    pub time: String,
    /// Hex-encoded SHA-256 over the canonical upload payload.
    #[serde(rename = "attestlyuploadhash")]
    pub upload_hash_hex: String,
    /// Length in bytes of the captured file payload. Useful for
    /// verifier-side sanity checks before recomputing the hash.
    #[serde(rename = "attestlyuploadbytes")]
    pub upload_bytes: u64,
    /// Identifier of the source capture tool (`"tella-android"`,
    /// `"tella-ios"`, etc.).
    #[serde(rename = "attestlysourcetool")]
    pub source_tool: String,
    /// Optional capture context (free-form human-readable tag).
    #[serde(
        rename = "attestlycapturecontext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_context: Option<String>,
    /// Attestly Decision Schema version (currently `"0.1"`).
    #[serde(rename = "attestlyschemaversion")]
    pub schema_version: String,
    /// Adapter-specific schema version
    /// (currently [`ADAPTER_SCHEMA_VERSION`]).
    #[serde(rename = "attestlyadapterversion")]
    pub adapter_version: String,
    /// CloudEvents `data` — the canonicalised Tella metadata blob.
    /// The file payload itself is *not* included here; only its hash.
    pub data: serde_json::Value,
}

impl FieldEvidenceEvent {
    /// Construct a `FieldEvidenceEvent` from a [`TellaUpload`] and the
    /// capturing organisation's DID (`did:web:...`).
    pub fn from_upload(upload: &TellaUpload, org_did: &str) -> Self {
        let upload_hash = canonical_upload_hash(upload);
        let upload_hash_hex = hex::encode(upload_hash);
        let id = format!("attestly-tella-{}", &upload_hash_hex[..16]);
        Self {
            id,
            source: org_did.to_string(),
            event_type: FIELD_EVIDENCE_EVENT_TYPE.to_string(),
            time: upload.captured_at.clone(),
            upload_hash_hex,
            upload_bytes: upload.file_bytes.len() as u64,
            source_tool: upload.source_tool.clone(),
            capture_context: upload.capture_context.clone(),
            schema_version: "0.1".to_string(),
            adapter_version: ADAPTER_SCHEMA_VERSION.to_string(),
            data: upload.metadata.clone(),
        }
    }
}

/// Canonical SHA-256 hash over a [`TellaUpload`].
///
/// The canonicalisation is deliberately simple:
/// `SHA-256(file_bytes || 0x1E || canonical_json(metadata) || 0x1E || captured_at || 0x1E || source_tool)`
///
/// `0x1E` is the ASCII Record Separator, used as a domain-separation
/// byte between fields. The full algorithm is documented in
/// `docs/protocol.md` (to be written under OTF M1) and frozen as part
/// of the adapter wire-format spec.
pub fn canonical_upload_hash(upload: &TellaUpload) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&upload.file_bytes);
    hasher.update([0x1E]);
    // The metadata JSON is canonicalised by serializing with serde_json
    // in deterministic key order. For a first-pass implementation we
    // use serde_json's default ordering; a future revision will adopt
    // RFC 8785 (JSON Canonicalization Scheme) once attestly-core moves
    // to that canonical form too. Both attestly-core and attestly-tella
    // must use the same JCS implementation when this lands.
    let metadata_canonical =
        serde_json::to_vec(&upload.metadata).unwrap_or_default();
    hasher.update(&metadata_canonical);
    hasher.update([0x1E]);
    hasher.update(upload.captured_at.as_bytes());
    hasher.update([0x1E]);
    hasher.update(upload.source_tool.as_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_upload() -> TellaUpload {
        TellaUpload::builder()
            .file_bytes(b"sample-evidence-bytes".to_vec())
            .metadata(serde_json::json!({"captured_at": "2026-06-01T10:00:00Z"}))
            .captured_at("2026-06-01T10:00:00Z")
            .source_tool("tella-android")
            .capture_context("protest-2026-06-01")
            .build()
            .unwrap()
    }

    #[test]
    fn canonical_hash_is_deterministic() {
        let upload = sample_upload();
        let h1 = canonical_upload_hash(&upload);
        let h2 = canonical_upload_hash(&upload);
        assert_eq!(h1, h2);
    }

    #[test]
    fn canonical_hash_detects_file_tamper() {
        let mut upload = sample_upload();
        let h_before = canonical_upload_hash(&upload);
        upload.file_bytes[0] ^= 0xff;
        let h_after = canonical_upload_hash(&upload);
        assert_ne!(h_before, h_after);
    }

    #[test]
    fn canonical_hash_detects_metadata_tamper() {
        let mut upload = sample_upload();
        let h_before = canonical_upload_hash(&upload);
        upload.metadata =
            serde_json::json!({"captured_at": "2099-01-01T00:00:00Z"});
        let h_after = canonical_upload_hash(&upload);
        assert_ne!(h_before, h_after);
    }

    #[test]
    fn canonical_hash_detects_source_tool_substitution() {
        let mut upload = sample_upload();
        let h_before = canonical_upload_hash(&upload);
        upload.source_tool = "different-capture-tool".to_string();
        let h_after = canonical_upload_hash(&upload);
        assert_ne!(h_before, h_after);
    }

    #[test]
    fn from_upload_populates_required_fields() {
        let upload = sample_upload();
        let evt = FieldEvidenceEvent::from_upload(&upload, "did:web:example.org");
        assert_eq!(evt.event_type, FIELD_EVIDENCE_EVENT_TYPE);
        assert_eq!(evt.source, "did:web:example.org");
        assert_eq!(evt.upload_bytes, upload.file_bytes.len() as u64);
        assert_eq!(evt.source_tool, upload.source_tool);
        assert_eq!(evt.capture_context.as_deref(), Some("protest-2026-06-01"));
        assert!(evt.id.starts_with("attestly-tella-"));
        assert_eq!(evt.upload_hash_hex.len(), 64);
    }

    #[test]
    fn from_upload_omits_file_bytes_from_event_data() {
        let upload = sample_upload();
        let evt = FieldEvidenceEvent::from_upload(&upload, "did:web:example.org");
        let serialised = serde_json::to_string(&evt).unwrap();
        // The raw file bytes must NOT appear in the event — only the
        // hash. This is the privacy guarantee of the adapter.
        assert!(!serialised.contains("sample-evidence-bytes"));
        assert!(serialised.contains(&evt.upload_hash_hex));
    }
}
