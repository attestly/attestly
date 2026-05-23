//! Attestly core: open verification infrastructure for EU AI Act Article 12 evidence.
//!
//! Three primitives:
//! 1. Cryptographic identity for AI systems (Ed25519 + did:web)
//! 2. Append-only signed event ledger (SQLite-backed in the demo; Postgres in production)
//! 3. Merkle-rooted signed checkpoints (Certificate-Transparency-style public commitments)

pub mod checkpoint;
pub mod error;
pub mod event;
pub mod identity;
pub mod ledger;
pub mod merkle;

pub use checkpoint::{InclusionProof, SignedCheckpoint};
pub use error::AttestlyError;
pub use event::{DecisionEvent, EventBuilder};
pub use identity::{OrgIdentity, SystemKey};
pub use ledger::{AppendReceipt, Ledger, StoredEvent};
