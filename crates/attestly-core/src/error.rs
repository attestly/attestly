use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttestlyError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("signature error: {0}")]
    Signature(#[from] ed25519_dalek::SignatureError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("hex decode error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("decision event missing required field: {0}")]
    EventMissingField(&'static str),

    #[error("checkpoint signature invalid")]
    CheckpointSignatureInvalid,

    #[error("tamper detected: {0}")]
    TamperDetected(String),

    #[error("invalid did: {0}")]
    InvalidDid(String),
}

pub type Result<T> = std::result::Result<T, AttestlyError>;
