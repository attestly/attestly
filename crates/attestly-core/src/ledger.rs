//! Append-only signed event ledger.
//!
//! The demo backend is SQLite (zero install — bundled via rusqlite).
//! Production will be Postgres; the schema and access pattern are
//! deliberately portable.
//!
//! Append-only enforcement: a SQLite trigger raises an exception on any
//! UPDATE or DELETE against the `audit_log` table. The trigger is part
//! of the migration; tamper attempts via the standard SQL surface fail.
//! (A determined operator with raw file access can edit the database
//! file — that's what the public Signed Tree Head catches. See README.)

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::error::{AttestlyError, Result};
use crate::event::DecisionEvent;
use crate::identity::SystemKey;

pub struct Ledger {
    conn: Connection,
    system_key: SystemKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendReceipt {
    pub seq: i64,
    pub event_id: String,
    /// Hex-encoded canonical hash of the event.
    pub payload_hash: String,
    /// Base64url-encoded Ed25519 signature over the payload hash.
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub seq: i64,
    pub system_did: String,
    pub event: DecisionEvent,
    /// Hex-encoded canonical hash.
    pub payload_hash: String,
    /// Hex-encoded previous entry hash (`None` only for seq=1).
    pub prev_hash: Option<String>,
    /// Base64url-encoded Ed25519 signature.
    pub signature: String,
    pub ts: String,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS audit_log (
    seq            INTEGER PRIMARY KEY AUTOINCREMENT,
    system_did     TEXT NOT NULL,
    event_id       TEXT NOT NULL UNIQUE,
    event_type     TEXT NOT NULL,
    category       TEXT NOT NULL,
    payload        TEXT NOT NULL,     -- JSON
    payload_hash   BLOB NOT NULL,     -- 32 bytes
    prev_hash      BLOB,              -- 32 bytes, NULL for first row
    sig            BLOB NOT NULL,     -- 64 bytes
    ts             TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE INDEX IF NOT EXISTS idx_audit_log_system_did_ts ON audit_log(system_did, ts);
CREATE INDEX IF NOT EXISTS idx_audit_log_category ON audit_log(category);

-- Append-only enforcement.
CREATE TRIGGER IF NOT EXISTS audit_log_no_update
BEFORE UPDATE ON audit_log
BEGIN
    SELECT RAISE(ABORT, 'audit_log is append-only; UPDATE forbidden');
END;

CREATE TRIGGER IF NOT EXISTS audit_log_no_delete
BEFORE DELETE ON audit_log
BEGIN
    SELECT RAISE(ABORT, 'audit_log is append-only; DELETE forbidden');
END;
"#;

impl Ledger {
    /// Open or create a SQLite ledger at `path`. Runs the schema migration
    /// idempotently.
    pub fn open(path: &str, system_key: SystemKey) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn, system_key })
    }

    /// Open an in-memory ledger (tests only).
    #[cfg(test)]
    pub fn open_in_memory(system_key: SystemKey) -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn, system_key })
    }

    pub fn system_did(&self) -> &str {
        &self.system_key.did
    }

    /// Append a decision event. Computes canonical hash, signs with the
    /// system key, inserts the row. The read of `last_hash` + the insert
    /// run inside an IMMEDIATE transaction so two concurrent appends
    /// cannot chain themselves to the same predecessor.
    pub fn append(&mut self, event: DecisionEvent) -> Result<AppendReceipt> {
        event.validate()?;
        let hash = event.canonical_hash()?;
        let sig = self.system_key.sign(&hash);
        let sig_bytes = sig.to_bytes();
        let payload_json = serde_json::to_string(&event)?;

        let tx = self
            .conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let prev_hash: Option<Vec<u8>> = tx
            .query_row(
                "SELECT payload_hash FROM audit_log ORDER BY seq DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .optional()?;
        tx.execute(
            "INSERT INTO audit_log
               (system_did, event_id, event_type, category, payload, payload_hash, prev_hash, sig)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                self.system_key.did,
                event.id,
                event.event_type,
                event.category,
                payload_json,
                &hash[..],
                prev_hash.as_deref(),
                &sig_bytes[..],
            ],
        )?;
        let seq = tx.last_insert_rowid();
        tx.commit()?;

        Ok(AppendReceipt {
            seq,
            event_id: event.id,
            payload_hash: hex::encode(hash),
            signature: base64_encode(&sig_bytes),
        })
    }

    /// Number of events in the ledger.
    pub fn tree_size(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM audit_log", [], |r| r.get(0))?;
        Ok(count as u64)
    }

    /// Read one stored event by sequence number.
    pub fn read(&self, seq: i64) -> Result<Option<StoredEvent>> {
        self.conn
            .query_row(
                "SELECT seq, system_did, payload, payload_hash, prev_hash, sig, ts
                 FROM audit_log WHERE seq = ?1",
                params![seq],
                |row| {
                    let payload: String = row.get(2)?;
                    let payload_hash: Vec<u8> = row.get(3)?;
                    let prev_hash: Option<Vec<u8>> = row.get(4)?;
                    let sig: Vec<u8> = row.get(5)?;
                    Ok(StoredEventRaw {
                        seq: row.get(0)?,
                        system_did: row.get(1)?,
                        payload,
                        payload_hash,
                        prev_hash,
                        sig,
                        ts: row.get(6)?,
                    })
                },
            )
            .optional()?
            .map(StoredEventRaw::into_stored)
            .transpose()
    }

    /// All payload hashes in order — used to rebuild the Merkle tree.
    pub fn all_hashes(&self) -> Result<Vec<[u8; 32]>> {
        let mut stmt = self
            .conn
            .prepare("SELECT payload_hash FROM audit_log ORDER BY seq")?;
        let rows = stmt.query_map([], |row| {
            let v: Vec<u8> = row.get(0)?;
            Ok(v)
        })?;
        let mut hashes = Vec::new();
        for row in rows {
            let v = row?;
            let arr: [u8; 32] = v
                .try_into()
                .map_err(|_| AttestlyError::TamperDetected("payload_hash not 32 bytes".into()))?;
            hashes.push(arr);
        }
        Ok(hashes)
    }

    /// Recompute the payload hash from the stored event JSON and compare
    /// against the stored hash. If they differ, the row has been tampered
    /// with at the file level (since the trigger blocks UPDATE/DELETE in
    /// SQL but cannot stop someone editing the file directly).
    pub fn recompute_hash(&self, seq: i64) -> Result<Option<[u8; 32]>> {
        let row: Option<(String, Vec<u8>)> = self
            .conn
            .query_row(
                "SELECT payload, payload_hash FROM audit_log WHERE seq = ?1",
                params![seq],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((payload, stored_hash)) = row else {
            return Ok(None);
        };
        let event: DecisionEvent = serde_json::from_str(&payload)?;
        let recomputed = event.canonical_hash()?;
        if recomputed != stored_hash.as_slice() {
            return Err(AttestlyError::TamperDetected(format!(
                "seq={seq}: stored hash {} does not match canonical hash {} of payload",
                hex::encode(&stored_hash),
                hex::encode(recomputed)
            )));
        }
        Ok(Some(recomputed))
    }
}

struct StoredEventRaw {
    seq: i64,
    system_did: String,
    payload: String,
    payload_hash: Vec<u8>,
    prev_hash: Option<Vec<u8>>,
    sig: Vec<u8>,
    ts: String,
}

impl StoredEventRaw {
    fn into_stored(self) -> Result<StoredEvent> {
        let event: DecisionEvent = serde_json::from_str(&self.payload)?;
        Ok(StoredEvent {
            seq: self.seq,
            system_did: self.system_did,
            event,
            payload_hash: hex::encode(&self.payload_hash),
            prev_hash: self.prev_hash.map(hex::encode),
            signature: base64_encode(&self.sig),
            ts: self.ts,
        })
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventBuilder;
    use crate::identity::SystemKey;

    fn make_event(n: u32) -> DecisionEvent {
        EventBuilder::new(
            "did:web:operator.example/ai-system/credit-scorer",
            "credit_score",
        )
        .model_id("model-v3.4.1")
        .data(serde_json::json!({"decision": "denied", "score": 0.41, "n": n}))
        .build()
    }

    #[test]
    fn append_persists_and_returns_receipt() {
        let key = SystemKey::generate("did:web:operator.example/ai-system/test");
        let mut ledger = Ledger::open_in_memory(key).unwrap();
        let r = ledger.append(make_event(0)).unwrap();
        assert_eq!(r.seq, 1);
        assert_eq!(ledger.tree_size().unwrap(), 1);
    }

    #[test]
    fn append_chains_prev_hash() {
        let key = SystemKey::generate("did:web:x");
        let mut ledger = Ledger::open_in_memory(key).unwrap();
        for n in 0..5 {
            ledger.append(make_event(n)).unwrap();
        }
        let e3 = ledger.read(3).unwrap().unwrap();
        let e4 = ledger.read(4).unwrap().unwrap();
        assert_eq!(e4.prev_hash.unwrap(), e3.payload_hash);
    }

    #[test]
    fn update_attempt_raises_append_only() {
        let key = SystemKey::generate("did:web:x");
        let mut ledger = Ledger::open_in_memory(key).unwrap();
        ledger.append(make_event(0)).unwrap();
        let err = ledger
            .conn
            .execute("UPDATE audit_log SET event_id = 'evil' WHERE seq = 1", [])
            .unwrap_err();
        assert!(err.to_string().contains("append-only"), "got: {err}");
    }

    #[test]
    fn delete_attempt_raises_append_only() {
        let key = SystemKey::generate("did:web:x");
        let mut ledger = Ledger::open_in_memory(key).unwrap();
        ledger.append(make_event(0)).unwrap();
        let err = ledger
            .conn
            .execute("DELETE FROM audit_log WHERE seq = 1", [])
            .unwrap_err();
        assert!(err.to_string().contains("append-only"), "got: {err}");
    }

    #[test]
    fn recompute_hash_matches_for_clean_row() {
        let key = SystemKey::generate("did:web:x");
        let mut ledger = Ledger::open_in_memory(key).unwrap();
        ledger.append(make_event(0)).unwrap();
        assert!(ledger.recompute_hash(1).unwrap().is_some());
    }
}
