// Attestly CLI — single binary `attestly` with subcommands.

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use attestly_core::checkpoint::{InclusionProof, SignedCheckpoint};
use attestly_core::event::{DecisionEvent, EventBuilder};
use attestly_core::identity::{OrgKey, SystemKey};
use attestly_core::ledger::Ledger;
use attestly_core::merkle;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "attestly",
    version,
    about = "EU AI Act Article 12 evidence integrity"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialise a new ledger + generate system and organisation keys.
    Init {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        /// System DID, e.g. did:web:bank.example/ai-system/credit-scorer
        #[arg(long)]
        system_did: String,
        /// Organisation DID, e.g. did:web:bank.example
        #[arg(long)]
        org_did: String,
        /// Key ID suffix for the org key, e.g. ops-2026-05
        #[arg(long, default_value = "ops-2026-05")]
        key_id: String,
        /// Directory to write keys + DID document
        #[arg(long, default_value = ".")]
        keys_dir: String,
    },

    /// Append a decision event. Reads CloudEvents JSON from stdin OR uses
    /// --data to build a synthetic credit-score decision.
    Append {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        #[arg(long, default_value = ".")]
        keys_dir: String,
        /// If set, builds a synthetic credit-score decision with this JSON `data`
        /// instead of reading a full CloudEvent from stdin.
        #[arg(long)]
        data: Option<String>,
    },

    /// Compute the current Merkle root and write a signed checkpoint.
    PublishCheckpoint {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        #[arg(long, default_value = ".")]
        keys_dir: String,
        #[arg(long)]
        out: String,
    },

    /// Export a regulator-facing evidence bundle for a single decision.
    Export {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        #[arg(long, default_value = ".")]
        keys_dir: String,
        #[arg(long = "decision-id")]
        decision_id: i64,
        /// Checkpoint JSON file to bundle (typically the latest published one).
        #[arg(long)]
        checkpoint: String,
        #[arg(long)]
        out: String,
    },

    /// Verify a regulator bundle. Exit 0 = valid, 1 = tampered, 2 = malformed.
    Verify {
        #[arg(long)]
        bundle: String,
    },

    /// Operator self-audit: recompute the canonical hash of every stored
    /// event and compare against the stored payload_hash column. Detects
    /// raw-file or admin-level tampering of the audit_log table.
    Audit {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        #[arg(long, default_value = ".")]
        keys_dir: String,
    },

    /// Demo-only: directly mutate a decision payload via raw SQL, bypassing
    /// the append-only trigger. Demonstrates that the public commitment
    /// detects tampering even when the operator has DB admin access.
    DemoTamper {
        #[arg(long, default_value = "attestly.db")]
        db: String,
        #[arg(long)]
        seq: i64,
        /// New value for the `decision` field in the JSON payload.
        #[arg(long, default_value = "approved")]
        new_decision: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init {
            db,
            system_did,
            org_did,
            key_id,
            keys_dir,
        } => cmd_init(&db, &system_did, &org_did, &key_id, &keys_dir),
        Command::Append { db, keys_dir, data } => cmd_append(&db, &keys_dir, data.as_deref()),
        Command::PublishCheckpoint { db, keys_dir, out } => {
            cmd_publish_checkpoint(&db, &keys_dir, &out)
        }
        Command::Export {
            db,
            keys_dir,
            decision_id,
            checkpoint,
            out,
        } => cmd_export(&db, &keys_dir, decision_id, &checkpoint, &out),
        Command::Verify { bundle } => cmd_verify(&bundle),
        Command::Audit { db, keys_dir } => cmd_audit(&db, &keys_dir),
        Command::DemoTamper {
            db,
            seq,
            new_decision,
        } => cmd_demo_tamper(&db, seq, &new_decision),
    }
}

fn cmd_audit(db: &str, keys_dir: &str) -> Result<()> {
    let keys_dir = PathBuf::from(keys_dir);
    let system_key = load_system_key(&keys_dir)?;
    let ledger = Ledger::open(db, system_key).context("opening ledger")?;
    let size = ledger.tree_size()? as i64;
    if size == 0 {
        println!("ledger empty");
        return Ok(());
    }
    let mut ok = 0i64;
    let mut bad = 0i64;
    for seq in 1..=size {
        match ledger.recompute_hash(seq) {
            Ok(Some(_)) => ok += 1,
            Ok(None) => bad += 1,
            Err(e) => {
                bad += 1;
                eprintln!("seq={seq}: {e}");
            }
        }
    }
    println!("audit: {ok} clean, {bad} mismatched (of {size} total)");
    if bad > 0 {
        std::process::exit(1);
    }
    Ok(())
}

// ---------- Command implementations ----------

fn cmd_init(db: &str, system_did: &str, org_did: &str, key_id: &str, keys_dir: &str) -> Result<()> {
    let keys_dir = PathBuf::from(keys_dir);
    fs::create_dir_all(&keys_dir).context("creating keys dir")?;

    // Generate the system key and persist its DID alongside the seed so
    // later commands (append, publish, export) can reload it.
    let system_key = SystemKey::generate(system_did);
    write_secret(&keys_dir.join("system.key"), &system_key.seed_b64())?;
    write_public(&keys_dir.join("system.pub"), &system_key.public_key_b64())?;
    fs::write(keys_dir.join("system.did"), system_did)?;

    // Initialise the ledger (schema migration is idempotent).
    let _ledger = Ledger::open(db, system_key).context("initialising ledger")?;

    // Generate the org key + DID document.
    let org_key = OrgKey::generate(org_did, key_id);
    write_secret(&keys_dir.join("org.key"), &org_key.seed_b64())?;
    write_public(
        &keys_dir.join("org.pub"),
        &org_key.identity.verifying_key_b64,
    )?;
    let did_doc = org_key.identity.did_document();
    fs::write(
        keys_dir.join("did.json"),
        serde_json::to_vec_pretty(&did_doc)?,
    )?;

    println!("✓ Initialised ledger at {db}");
    println!("✓ System DID:    {system_did}");
    println!("✓ Org DID:       {org_did}");
    println!("✓ Org key ID:    {}", org_key.identity.key_id);
    println!(
        "✓ Keys written:  {}/{{system,org}}.{{key,pub}}",
        keys_dir.display()
    );
    println!("✓ DID document:  {}/did.json", keys_dir.display());
    Ok(())
}

fn cmd_append(db: &str, keys_dir: &str, synthetic_data: Option<&str>) -> Result<()> {
    let keys_dir = PathBuf::from(keys_dir);
    let system_key = load_system_key(&keys_dir)?;
    let mut ledger = Ledger::open(db, system_key).context("opening ledger")?;
    let system_did = ledger.system_did().to_string();

    let event = if let Some(data_str) = synthetic_data {
        let data: serde_json::Value = serde_json::from_str(data_str).context("parsing --data")?;
        EventBuilder::new(&system_did, "credit_score")
            .model_id("model-v3.4.1")
            .data(data)
            .build()
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("reading stdin")?;
        serde_json::from_str::<DecisionEvent>(&buf).context("parsing CloudEvent JSON")?
    };

    let receipt = ledger.append(event)?;
    println!("{}", serde_json::to_string_pretty(&receipt)?);
    Ok(())
}

fn cmd_publish_checkpoint(db: &str, keys_dir: &str, out: &str) -> Result<()> {
    let keys_dir = PathBuf::from(keys_dir);
    let system_key = load_system_key(&keys_dir)?;
    let org_key = load_org_key(&keys_dir)?;
    let ledger = Ledger::open(db, system_key).context("opening ledger")?;
    let hashes = ledger.all_hashes()?;
    if hashes.is_empty() {
        bail!("ledger is empty — nothing to publish");
    }
    let tree = merkle::build_tree(&hashes);
    let root = merkle::root(&tree);
    let checkpoint = SignedCheckpoint::build(&org_key, hashes.len() as u64, root);
    fs::write(out, serde_json::to_vec_pretty(&checkpoint)?)?;
    println!("✓ Published checkpoint: {out}");
    println!("  tree_size: {}", checkpoint.tree_size);
    println!("  root:      {}", checkpoint.root_hash);
    println!("  signed by: {}", checkpoint.signature.key_id);
    Ok(())
}

fn cmd_export(
    db: &str,
    keys_dir: &str,
    decision_id: i64,
    checkpoint_path: &str,
    out: &str,
) -> Result<()> {
    let keys_dir = PathBuf::from(keys_dir);
    let system_key = load_system_key(&keys_dir)?;
    let ledger = Ledger::open(db, system_key).context("opening ledger")?;

    let checkpoint_bytes = fs::read(checkpoint_path).context("reading checkpoint")?;
    let checkpoint: SignedCheckpoint =
        serde_json::from_slice(&checkpoint_bytes).context("parsing checkpoint")?;

    let stored = ledger
        .read(decision_id)?
        .ok_or_else(|| anyhow!("decision seq={decision_id} not found in ledger"))?;

    // Recompute canonical hash from the stored payload. If the operator has
    // tampered with the stored event after publication, this will differ
    // from the leaf hash committed in the published checkpoint — that's
    // exactly what we want to expose to the regulator.
    let recomputed = stored.event.canonical_hash()?;
    let recomputed_hex = hex::encode(recomputed);

    // Rebuild Merkle tree to produce the inclusion proof.
    let hashes = ledger.all_hashes()?;
    if (checkpoint.tree_size as usize) > hashes.len() {
        bail!(
            "checkpoint references tree_size={} but ledger only has {} entries",
            checkpoint.tree_size,
            hashes.len()
        );
    }
    // Build the tree at the checkpoint's tree_size — this is the historical
    // tree the checkpoint committed to.
    let hashes_at_checkpoint: Vec<[u8; 32]> = hashes
        .into_iter()
        .take(checkpoint.tree_size as usize)
        .collect();
    let tree = merkle::build_tree(&hashes_at_checkpoint);

    let leaf_index = (decision_id as usize)
        .checked_sub(1)
        .ok_or_else(|| anyhow!("decision seq must be >= 1"))?;
    if leaf_index >= hashes_at_checkpoint.len() {
        bail!(
            "decision seq={decision_id} not included in checkpoint (tree_size={})",
            checkpoint.tree_size
        );
    }

    // The leaf hash in the proof is the canonical hash at the time the
    // checkpoint was published (== the stored hash, since we hash on
    // append). The export *also* embeds the current canonical hash
    // alongside the event payload — when the verifier re-canonicalizes
    // the payload and compares, tampering surfaces.
    let leaf_hash_at_checkpoint = hashes_at_checkpoint[leaf_index];
    let proof_data = merkle::inclusion_proof(&tree, leaf_index);

    let inclusion = InclusionProof {
        leaf_seq: decision_id,
        leaf_hash: hex::encode(leaf_hash_at_checkpoint),
        proof: proof_data,
        checkpoint_origin: checkpoint.origin.clone(),
        checkpoint_tree_size: checkpoint.tree_size,
        checkpoint_root_hash: checkpoint.root_hash.clone(),
    };

    // Read the DID document (operator's public key).
    let did_doc_path = keys_dir.join("did.json");
    let did_doc: serde_json::Value =
        serde_json::from_slice(&fs::read(&did_doc_path).context("reading did.json")?)?;

    // Build the bundle ZIP.
    let zip_file = fs::File::create(out).context("creating bundle file")?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let opts: zip::write::SimpleFileOptions = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let readme = render_readme(&checkpoint, decision_id, &recomputed_hex, &stored.event.id);
    add_text(&mut zip, "README.md", &readme, opts)?;
    add_json(&mut zip, "decision.json", &stored.event, opts)?;
    add_json(&mut zip, "inclusion-proof.json", &inclusion, opts)?;
    add_json(&mut zip, "checkpoint.json", &checkpoint, opts)?;
    add_json(&mut zip, "did-document.json", &did_doc, opts)?;
    zip.finish().context("finalising zip")?;

    println!("✓ Exported regulator bundle: {out}");
    println!("  decision seq:   {decision_id}");
    println!("  event id:       {}", stored.event.id);
    println!("  leaf @ ckpt:    {}", inclusion.leaf_hash);
    println!("  payload now:    {recomputed_hex}");
    if recomputed_hex != inclusion.leaf_hash {
        println!(
            "  ⚠️  payload differs from committed leaf — verifier WILL fail (intended for demo)"
        );
    }
    Ok(())
}

fn cmd_verify(bundle_path: &str) -> Result<()> {
    let path = std::path::Path::new(bundle_path);
    let result = attestly_verifier::verify_bundle(path);
    match &result {
        attestly_verifier::VerifyResult::Valid {
            decision_seq,
            checkpoint_origin,
            checkpoint_tree_size,
        } => {
            println!("[OK]    decision seq={decision_seq} verified against the public commitment.");
            println!("        operator:    {checkpoint_origin}");
            println!("        tree_size:   {checkpoint_tree_size}");
            println!("        This is the decision record at the time of publication.");
        }
        attestly_verifier::VerifyResult::Tampered {
            reason,
            expected,
            actual,
        } => {
            eprintln!("[FAIL]  TAMPERED — {reason}");
            eprintln!("        expected: {expected}");
            eprintln!("        actual:   {actual}");
        }
        attestly_verifier::VerifyResult::Malformed { reason } => {
            eprintln!("[ERR]   bundle malformed: {reason}");
        }
    }
    std::process::exit(result.exit_code());
}

fn cmd_demo_tamper(db: &str, seq: i64, new_decision: &str) -> Result<()> {
    // Bypass the append-only trigger explicitly. This simulates a malicious
    // operator with DB admin access editing the audit trail after the fact.
    // The Attestly verifier still detects this because the published
    // Signed Tree Head committed to the canonical hash at publication time.
    let conn = rusqlite::Connection::open(db).context("opening db")?;
    conn.execute_batch(
        r#"
        DROP TRIGGER IF EXISTS audit_log_no_update;
        DROP TRIGGER IF EXISTS audit_log_no_delete;
        "#,
    )?;
    let payload: String = conn.query_row(
        "SELECT payload FROM audit_log WHERE seq = ?1",
        rusqlite::params![seq],
        |r| r.get(0),
    )?;
    let mut value: serde_json::Value = serde_json::from_str(&payload)?;
    if let Some(data) = value.get_mut("data").and_then(|v| v.as_object_mut()) {
        data.insert(
            "decision".into(),
            serde_json::Value::String(new_decision.into()),
        );
    }
    let new_payload = serde_json::to_string(&value)?;
    let updated = conn.execute(
        "UPDATE audit_log SET payload = ?1 WHERE seq = ?2",
        rusqlite::params![new_payload, seq],
    )?;
    conn.execute_batch(
        r#"
        CREATE TRIGGER audit_log_no_update
        BEFORE UPDATE ON audit_log
        BEGIN
            SELECT RAISE(ABORT, 'audit_log is append-only; UPDATE forbidden');
        END;
        CREATE TRIGGER audit_log_no_delete
        BEFORE DELETE ON audit_log
        BEGIN
            SELECT RAISE(ABORT, 'audit_log is append-only; DELETE forbidden');
        END;
        "#,
    )?;
    println!(
        "⚠️  Tampered seq={seq}: set data.decision='{new_decision}' (rows affected: {updated})"
    );
    println!("    triggers restored. The next `attestly export` + `attestly verify`");
    println!("    cycle will catch this mutation via the Merkle inclusion proof.");
    Ok(())
}

// ---------- helpers ----------

fn load_system_key(keys_dir: &std::path::Path) -> Result<SystemKey> {
    let seed = read_secret(&keys_dir.join("system.key"))?;
    let did = fs::read_to_string(keys_dir.join("system.did"))
        .with_context(|| format!("reading {}/system.did", keys_dir.display()))?
        .trim()
        .to_string();
    Ok(SystemKey::from_seed(did, seed))
}

fn load_org_key(keys_dir: &std::path::Path) -> Result<OrgKey> {
    let seed = read_secret(&keys_dir.join("org.key"))?;
    let did_doc: serde_json::Value =
        serde_json::from_slice(&fs::read(keys_dir.join("did.json")).context("did.json")?)?;
    let did = did_doc["id"]
        .as_str()
        .ok_or_else(|| anyhow!("did.json missing 'id'"))?
        .to_string();
    let key_id = did_doc["verificationMethod"][0]["id"]
        .as_str()
        .ok_or_else(|| anyhow!("did.json missing verificationMethod[0].id"))?;
    let key_id_suffix = key_id
        .split_once('#')
        .map(|(_, s)| s.to_string())
        .unwrap_or_else(|| "ops".to_string());
    Ok(OrgKey::from_seed(did, &key_id_suffix, seed))
}

fn read_secret(path: &std::path::Path) -> Result<[u8; 32]> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let bytes = URL_SAFE_NO_PAD.decode(raw.trim())?;
    bytes
        .try_into()
        .map_err(|_| anyhow!("key file must decode to 32 bytes: {}", path.display()))
}

fn write_secret(path: &std::path::Path, b64: &str) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    f.write_all(b64.as_bytes())?;
    Ok(())
}

fn write_public(path: &std::path::Path, b64: &str) -> Result<()> {
    fs::write(path, b64)?;
    Ok(())
}

fn add_text<W: Write + io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    name: &str,
    content: &str,
    opts: zip::write::SimpleFileOptions,
) -> Result<()> {
    zip.start_file(name, opts)?;
    zip.write_all(content.as_bytes())?;
    Ok(())
}

fn add_json<W: Write + io::Seek, T: serde::Serialize>(
    zip: &mut zip::ZipWriter<W>,
    name: &str,
    value: &T,
    opts: zip::write::SimpleFileOptions,
) -> Result<()> {
    zip.start_file(name, opts)?;
    zip.write_all(&serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

fn render_readme(
    checkpoint: &SignedCheckpoint,
    decision_seq: i64,
    payload_hash_now: &str,
    event_id: &str,
) -> String {
    format!(
        r#"# Attestly evidence bundle — regulator verification instructions

Decision sequence: {decision_seq}
Event id:          {event_id}
Operator DID:      {origin}
Tree size at checkpoint: {tree_size}
Checkpoint root hash:    {root}

## Files in this bundle

- `decision.json`         — the AI decision record as exported by the operator
- `inclusion-proof.json`  — Merkle inclusion proof against the public checkpoint
- `checkpoint.json`       — Signed Tree Head published by the operator at the time of the decision
- `did-document.json`     — the operator's public-key DID Document
- `README.md`             — this file

## How to verify

1. Install `attestly` (Rust toolchain):

       cargo install --git https://github.com/attestly/attestly attestly-cli

2. Run the verifier:

       attestly verify --bundle <this-bundle.zip>

The verifier:

- recomputes the canonical hash of `decision.json`,
- checks the inclusion proof against the checkpoint root,
- verifies the Ed25519 signature on the checkpoint using the public key in `did-document.json`.

If any step fails, the verifier prints a `[FAIL] TAMPERED` line explaining
which check failed.

## What this proves

The published Signed Tree Head is a cryptographic commitment to every
decision in the ledger at the time of publication. A regulator can verify
that the decision currently being shown in `decision.json` matches the
record the operator committed to publicly at `{timestamp}`.

If the operator has altered `decision.json` after publication, the
verifier will detect it because the canonical hash of the altered payload
will not match the leaf hash committed in the Merkle tree.

Current canonical hash of `decision.json`: {payload_hash_now}
"#,
        decision_seq = decision_seq,
        event_id = event_id,
        origin = checkpoint.origin,
        tree_size = checkpoint.tree_size,
        root = checkpoint.root_hash,
        timestamp = checkpoint.timestamp,
        payload_hash_now = payload_hash_now,
    )
}
