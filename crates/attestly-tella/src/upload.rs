//! Types describing a Tella upload payload as received by the Attestly
//! adapter.
//!
//! These types intentionally model the shape Tella's existing upload
//! pipeline produces — they are not a re-export of Tella's internal
//! types (we don't depend on Tella's codebase). The shape mirrors
//! Tella's documented Uwazi/ODK upload formats and the fields any
//! Tella deployment routes through to its destination backend.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ADAPTER_SCHEMA_VERSION;

/// A single upload payload from Tella, ready for Attestly attestation.
///
/// The adapter does not modify the file bytes or the metadata. It only
/// computes a canonical hash and appends a [`FieldEvidenceEvent`] to
/// the operator's Attestly ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TellaUpload {
    /// The captured-media payload — encrypted by Tella's existing
    /// at-rest encryption, OR plaintext if the deployment opts out of
    /// at-rest encryption.
    pub file_bytes: Vec<u8>,
    /// Tella-attached metadata as a free-form JSON object. Typically
    /// includes capture timestamp, optional coordinates, device
    /// information, and capture context.
    pub metadata: serde_json::Value,
    /// Capture timestamp in RFC 3339. Distinct from the timestamp
    /// inside `metadata` in case the adapter needs to assert capture
    /// time independent of Tella's own metadata claims.
    pub captured_at: String,
    /// Identifier of the source capture tool, e.g. `"tella-android"`,
    /// `"tella-ios"`, `"tella-web"`. Used so a single Attestly ledger
    /// can attest evidence from multiple capture-tool deployments.
    pub source_tool: String,
    /// Optional human-readable capture context — for example a
    /// short tag the documenter applied at capture time
    /// (`"protest-2026-06-01"`). Never includes location or
    /// identifying information beyond what the operator chose to
    /// attach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_context: Option<String>,
    /// Schema version the adapter understands. Currently always
    /// [`ADAPTER_SCHEMA_VERSION`].
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
}

fn default_schema_version() -> String {
    ADAPTER_SCHEMA_VERSION.to_string()
}

impl TellaUpload {
    /// Convenience builder.
    pub fn builder() -> TellaUploadBuilder {
        TellaUploadBuilder::default()
    }
}

/// Builder for [`TellaUpload`].
#[derive(Debug, Default)]
pub struct TellaUploadBuilder {
    file_bytes: Option<Vec<u8>>,
    metadata: Option<serde_json::Value>,
    captured_at: Option<String>,
    source_tool: Option<String>,
    capture_context: Option<String>,
}

impl TellaUploadBuilder {
    /// Set the file bytes (captured media payload).
    pub fn file_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.file_bytes = Some(bytes);
        self
    }

    /// Set the Tella metadata blob.
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set the captured-at timestamp. If not set, the adapter uses
    /// the current UTC time at build time.
    pub fn captured_at(mut self, ts: impl Into<String>) -> Self {
        self.captured_at = Some(ts.into());
        self
    }

    /// Set the source-tool identifier (`"tella-android"`, etc.).
    pub fn source_tool(mut self, tool: impl Into<String>) -> Self {
        self.source_tool = Some(tool.into());
        self
    }

    /// Set the optional human-readable capture context.
    pub fn capture_context(mut self, ctx: impl Into<String>) -> Self {
        self.capture_context = Some(ctx.into());
        self
    }

    /// Build the [`TellaUpload`]. Returns an error if required fields
    /// were not set.
    pub fn build(self) -> Result<TellaUpload, BuildError> {
        let file_bytes = self.file_bytes.ok_or(BuildError::MissingFileBytes)?;
        let metadata = self.metadata.unwrap_or(serde_json::Value::Null);
        let captured_at = self.captured_at.unwrap_or_else(|| {
            // Format current UTC time as RFC 3339.
            OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
        });
        let source_tool = self
            .source_tool
            .ok_or(BuildError::MissingSourceTool)?;
        Ok(TellaUpload {
            file_bytes,
            metadata,
            captured_at,
            source_tool,
            capture_context: self.capture_context,
            schema_version: ADAPTER_SCHEMA_VERSION.to_string(),
        })
    }
}

/// Errors that can occur when building a [`TellaUpload`].
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    /// `file_bytes` is required.
    #[error("file_bytes is required")]
    MissingFileBytes,
    /// `source_tool` is required.
    #[error("source_tool is required (e.g. \"tella-android\")")]
    MissingSourceTool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_file_bytes() {
        let err = TellaUpload::builder()
            .source_tool("tella-android")
            .build()
            .unwrap_err();
        assert!(matches!(err, BuildError::MissingFileBytes));
    }

    #[test]
    fn builder_requires_source_tool() {
        let err = TellaUpload::builder()
            .file_bytes(b"hi".to_vec())
            .build()
            .unwrap_err();
        assert!(matches!(err, BuildError::MissingSourceTool));
    }

    #[test]
    fn builder_constructs_minimal_upload() {
        let upload = TellaUpload::builder()
            .file_bytes(b"hi".to_vec())
            .source_tool("tella-android")
            .build()
            .unwrap();
        assert_eq!(upload.file_bytes, b"hi");
        assert_eq!(upload.source_tool, "tella-android");
        assert_eq!(upload.schema_version, ADAPTER_SCHEMA_VERSION);
        assert!(upload.capture_context.is_none());
    }

    #[test]
    fn builder_serializes_round_trip() {
        let upload = TellaUpload::builder()
            .file_bytes(b"hi".to_vec())
            .source_tool("tella-android")
            .metadata(serde_json::json!({"k": "v"}))
            .capture_context("protest-2026-06-01")
            .build()
            .unwrap();
        let s = serde_json::to_string(&upload).unwrap();
        let back: TellaUpload = serde_json::from_str(&s).unwrap();
        assert_eq!(back.source_tool, upload.source_tool);
        assert_eq!(back.capture_context, upload.capture_context);
    }
}
