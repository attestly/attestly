//! Decision event — the unit of evidence appended to the Attestly ledger.
//!
//! Events follow a CloudEvents v1.0-compatible shape with Attestly
//! extension attributes for AI Act Annex III category routing.
//!
//! Canonical hashing uses a deterministic JSON serialization (sorted keys,
//! UTF-8, no whitespace) so the same logical event produces the same hash
//! regardless of field ordering in transit.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::{AttestlyError, Result};

/// A decision event, ready to be appended to the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEvent {
    /// CloudEvents `id` — globally unique per event.
    pub id: String,
    /// CloudEvents `source` — the AI system DID that produced the decision.
    pub source: String,
    /// CloudEvents `type` — typically `"ai.attestly.decision.v1"`.
    #[serde(rename = "type")]
    pub event_type: String,
    /// CloudEvents `time` (RFC 3339).
    pub time: String,
    /// AI Act Annex III category (e.g. `"credit_score"`, `"employment"`).
    #[serde(rename = "attestlydecisioncategory")]
    pub category: String,
    /// Attestly schema version (currently `"0.1"`).
    #[serde(rename = "attestlyschemaversion")]
    pub schema_version: String,
    /// Model identifier (operator-defined).
    #[serde(
        rename = "attestlymodelid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model_id: Option<String>,
    /// Pseudonymous hashed subject reference (operator-controlled).
    #[serde(
        rename = "attestlysubjectref",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_ref: Option<String>,
    /// CloudEvents `data` — the decision payload itself. JSON object.
    pub data: serde_json::Value,
}

impl DecisionEvent {
    /// Compute the canonical SHA-256 hash used for ledger appending.
    ///
    /// Canonicalization: serialize the event as JSON with sorted keys
    /// (via [`serde_canonical`]-style ordering), no whitespace, UTF-8.
    /// SHA-256 over the UTF-8 bytes.
    pub fn canonical_hash(&self) -> Result<[u8; 32]> {
        let value = serde_json::to_value(self)?;
        let canonical = canonicalize(&value);
        Ok(Sha256::digest(canonical.as_bytes()).into())
    }

    /// Validate that required Attestly extension attributes are present
    /// and well-formed. Lightweight; not a full JSON Schema validation.
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(AttestlyError::EventMissingField("id"));
        }
        if self.source.is_empty() {
            return Err(AttestlyError::EventMissingField("source"));
        }
        if self.event_type.is_empty() {
            return Err(AttestlyError::EventMissingField("type"));
        }
        if self.time.is_empty() {
            return Err(AttestlyError::EventMissingField("time"));
        }
        if self.category.is_empty() {
            return Err(AttestlyError::EventMissingField("attestlydecisioncategory"));
        }
        if self.schema_version.is_empty() {
            return Err(AttestlyError::EventMissingField("attestlyschemaversion"));
        }
        Ok(())
    }
}

/// Fluent builder for [`DecisionEvent`]. Auto-generates `id` and `time` if not supplied.
pub struct EventBuilder {
    source: String,
    category: String,
    event_type: String,
    schema_version: String,
    model_id: Option<String>,
    subject_ref: Option<String>,
    data: serde_json::Value,
}

impl EventBuilder {
    pub fn new(source: impl Into<String>, category: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            category: category.into(),
            event_type: "ai.attestly.decision.v1".into(),
            schema_version: "0.1".into(),
            model_id: None,
            subject_ref: None,
            data: serde_json::Value::Object(Default::default()),
        }
    }

    pub fn model_id(mut self, model_id: impl Into<String>) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    pub fn subject_ref(mut self, subject_ref: impl Into<String>) -> Self {
        self.subject_ref = Some(subject_ref.into());
        self
    }

    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) -> DecisionEvent {
        let time = OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .expect("rfc3339 format");
        DecisionEvent {
            id: Uuid::new_v4().to_string(),
            source: self.source,
            event_type: self.event_type,
            time,
            category: self.category,
            schema_version: self.schema_version,
            model_id: self.model_id,
            subject_ref: self.subject_ref,
            data: self.data,
        }
    }
}

/// Canonical JSON serialization: keys sorted recursively, no whitespace.
/// This is the operation that makes two events produce identical hashes
/// regardless of upstream serializer key ordering.
fn canonicalize(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut entries: Vec<(&String, &serde_json::Value)> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let mut s = String::from("{");
            for (i, (k, v)) in entries.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&serde_json::to_string(k).expect("json string"));
                s.push(':');
                s.push_str(&canonicalize(v));
            }
            s.push('}');
            s
        }
        serde_json::Value::Array(arr) => {
            let mut s = String::from("[");
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&canonicalize(v));
            }
            s.push(']');
            s
        }
        _ => serde_json::to_string(value).expect("json value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_valid_event() {
        let event = EventBuilder::new(
            "did:web:operator.example/ai-system/credit-scorer",
            "credit_score",
        )
        .model_id("model-v3.4.1")
        .data(serde_json::json!({"decision": "denied", "score": 0.41}))
        .build();
        event.validate().unwrap();
        assert_eq!(event.event_type, "ai.attestly.decision.v1");
        assert_eq!(event.schema_version, "0.1");
    }

    #[test]
    fn canonical_hash_is_stable() {
        // Two events with identical content but constructed in different
        // orders must produce the same canonical hash.
        let raw1 = serde_json::json!({
            "id": "abc",
            "source": "did:web:x",
            "type": "ai.attestly.decision.v1",
            "time": "2026-05-21T15:00:00Z",
            "attestlydecisioncategory": "credit_score",
            "attestlyschemaversion": "0.1",
            "data": {"a": 1, "b": 2}
        });
        let raw2 = serde_json::json!({
            "data": {"b": 2, "a": 1},
            "attestlyschemaversion": "0.1",
            "attestlydecisioncategory": "credit_score",
            "time": "2026-05-21T15:00:00Z",
            "type": "ai.attestly.decision.v1",
            "source": "did:web:x",
            "id": "abc"
        });
        let e1: DecisionEvent = serde_json::from_value(raw1).unwrap();
        let e2: DecisionEvent = serde_json::from_value(raw2).unwrap();
        assert_eq!(e1.canonical_hash().unwrap(), e2.canonical_hash().unwrap());
    }

    #[test]
    fn canonical_hash_changes_when_payload_changes() {
        let e1 = EventBuilder::new("did:web:x", "credit_score")
            .data(serde_json::json!({"decision": "denied"}))
            .build();
        let mut e2 = e1.clone();
        e2.data = serde_json::json!({"decision": "approved"});
        assert_ne!(e1.canonical_hash().unwrap(), e2.canonical_hash().unwrap());
    }

    #[test]
    fn validate_rejects_empty_required_fields() {
        let mut event = EventBuilder::new("did:web:x", "credit_score").build();
        event.source = String::new();
        assert!(event.validate().is_err());
    }
}
