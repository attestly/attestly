//! attestly-tella — Attestly adapter for Tella secure-capture pipelines.
//!
//! This crate provides a thin integration layer between Tella's existing
//! upload pipeline and the Attestly append-only signed ledger. It does
//! not modify Tella itself; it sits between Tella's upload payload and
//! the destination backend (Uwazi / ODK / Nextcloud / etc.).
//!
//! See `README.md` for the architectural overview, and `docs/protocol.md`
//! (to be written under OTF M1) for the wire protocol.
//!
//! # Example
//!
//! ```no_run
//! use attestly_tella::{TellaAdapter, TellaUpload};
//! # use attestly_core::{Ledger, OrgIdentity};
//! # let ledger: Ledger = unimplemented!();
//! # let org: OrgIdentity = unimplemented!();
//!
//! let adapter = TellaAdapter::new(ledger, org);
//! let upload = TellaUpload::builder()
//!     .file_bytes(b"<encrypted media blob>".to_vec())
//!     .metadata(serde_json::json!({"captured_at": "2026-06-01T10:23:11Z"}))
//!     .source_tool("tella-android")
//!     .build()
//!     .unwrap();
//!
//! let receipt = adapter.attest(&upload).unwrap();
//! assert!(!receipt.event_id.is_empty());
//! ```

#![deny(missing_docs)]

pub mod adapter;
pub mod evidence;
pub mod receipt;
pub mod upload;

pub use adapter::TellaAdapter;
pub use evidence::FieldEvidenceEvent;
pub use receipt::AttestlyReceipt;
pub use upload::{TellaUpload, TellaUploadBuilder};

/// The Attestly Decision Schema event type used for Tella field-evidence
/// uploads. Parallel to the `ai.attestly.decision.v1` type used for AI
/// system decisions.
pub const FIELD_EVIDENCE_EVENT_TYPE: &str = "civil.attestly.evidence.v1";

/// Schema version of the Tella adapter wire format. Bumped on
/// incompatible changes.
pub const ADAPTER_SCHEMA_VERSION: &str = "0.0";
