# Attestly — Coding Sprint (Phase 0 → Phase 2)

> **Scope**: complete coding plan for Attestly through v1.0. Companion to `SPRINT-PLAN.md` (grant strategy). This doc is task-level for Phase 0 (Day 2-5) and story-level with acceptance criteria for Phase 1 (W1-6) + Phase 2 (W7-18). Funder retarget 2026-05-21: STF primary, EIC Step 1 parallel, NLnet Oct deferred backup.
>
> **Reading order**: §0 conventions → §1 repo → §2 stack → §3 CI → §4 tests → §5 specs → §6 perf budgets → §7-10 Phase 0 day-by-day → §11-14 Phase 1 weeks → §15-23 Phase 2 weeks → §24 perf at v1.0 → §25 what you own.

---

## 0. Conventions and non-negotiables

### 0.1 Toolchains
- **Rust**: stable 1.85+ (`rust-toolchain.toml` pins). Workspace uses 2024 edition.
- **Python**: 3.12 minimum. SDK supports 3.11+.
- **Node**: 20 LTS. SDK supports 18+.
- **Postgres**: 14 minimum (target 16 for new dev).
- **OS matrix**: Linux x86_64, macOS arm64, Windows x86_64. CI proves all three.

### 0.2 Lint + format (zero warnings)
- Rust: `cargo fmt --check` + `cargo clippy --all-targets --all-features -- -D warnings` must pass.
- Python: `ruff check` + `ruff format --check`. Mypy strict mode on the SDK.
- TypeScript: `tsc --noEmit --strict` + `biome check`.
- SQL: `sqlfluff lint --dialect postgres`.
- Markdown: `markdownlint-cli2`.
- Pre-commit hook runs all the above on staged files.

### 0.3 Commits + branches
- Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`).
- `main` is always green and demo-able. PRs only.
- Feature branches: `feat/<short-slug>`.
- Squash-merge to main; preserve the PR title as the squash message.
- Tag releases as `v<MAJOR>.<MINOR>.<PATCH>` (semver). `v0.1.0` ships at end of Day 5.

### 0.4 Definition of Done (every task)
A task is done when:
1. Code merged to `main`.
2. Tests added (unit + at least one integration scenario for behaviour tasks).
3. CI green on all three OSes.
4. Public-facing changes documented (`rustdoc`, Python docstring, or `docs/`).
5. If touches the wire format: spec file in `spec/` updated and JSON Schema validates.
6. If touches performance-critical paths: criterion benchmark added or updated.

### 0.5 What we explicitly DON'T do in Phase 0
- No Tessera integration (Phase 2, W9).
- No TypeScript SDK (Phase 2, W7-8).
- No web UI (Phase 2, W10).
- No RFC 3161 timestamping (Phase 3+).
- No external KMS support (Phase 2+).
- No federation between operators (out of v1.0 scope).
- No standards-body submission (Phase 2, W15-16).

---

## 1. Repo structure (lock Day 1)

```
attestly/
├── Cargo.toml                       # workspace
├── rust-toolchain.toml              # stable 1.85
├── .github/
│   └── workflows/
│       ├── ci.yml                   # lint + test matrix
│       ├── release.yml              # tag → publish
│       └── spec-publish.yml         # spec/ → docs site
├── crates/
│   ├── attestly-core/               # ledger, merkle, checkpoint
│   │   ├── src/lib.rs
│   │   ├── src/ledger.rs            # append + read API
│   │   ├── src/merkle.rs            # tree + proofs (rs_merkle wrap)
│   │   ├── src/checkpoint.rs        # SignedCheckpoint
│   │   ├── src/identity.rs          # ed25519 + did:web
│   │   ├── src/event.rs             # CloudEvent + Decision Schema
│   │   ├── src/error.rs             # AttestlyError
│   │   ├── migrations/              # sqlx migrations
│   │   │   └── 0001_audit_log.sql
│   │   ├── tests/
│   │   │   ├── append.rs
│   │   │   ├── tamper_detection.rs
│   │   │   └── property.rs
│   │   └── benches/
│   │       └── append_throughput.rs
│   ├── attestly-cli/                # binary: `attestly` CLI
│   │   ├── src/main.rs
│   │   ├── src/cmd/append.rs
│   │   ├── src/cmd/publish.rs
│   │   ├── src/cmd/export.rs
│   │   ├── src/cmd/verify.rs
│   │   └── tests/cli_smoke.rs
│   ├── attestly-verifier/           # standalone verifier (no DB deps)
│   │   ├── src/lib.rs
│   │   ├── src/bundle.rs            # parse regulator-N.zip
│   │   └── tests/golden.rs
│   └── attestly-pyo3/               # PyO3 bindings
│       ├── Cargo.toml
│       ├── pyproject.toml
│       └── src/lib.rs
├── sdk-python/
│   ├── pyproject.toml               # uses attestly-pyo3 wheel
│   ├── attestly/__init__.py
│   ├── attestly/ledger.py           # high-level wrapper
│   ├── attestly/cloudevents.py      # CloudEvents helpers
│   ├── attestly/adapters/
│   │   ├── __init__.py
│   │   ├── claude_agent_sdk.py      # Phase 1
│   │   └── openai_agents.py         # Phase 1
│   └── tests/
│       ├── test_ledger.py
│       ├── test_cloudevents.py
│       └── test_adapters.py
├── sdk-typescript/                   # scaffolded Day 1, fleshed out Phase 2
├── spec/
│   ├── decision-schema-v0.1.md
│   ├── decision-schema-v0.1.json    # JSON Schema 2020-12
│   ├── checkpoint-format-v0.1.md
│   ├── checkpoint-format-v0.1.json
│   ├── verifier-protocol-v0.1.md
│   └── article12-mapping.md
├── examples/
│   ├── credit_score_decision.py
│   ├── employment_screening.py      # Phase 1
│   └── agt_bridge.py                # Phase 2
├── docs/                             # mkdocs material (Phase 2)
│   ├── mkdocs.yml
│   └── docs/
├── corpus/                           # signed sample bundles for compliance demos
│   ├── credit-score-bundle-001.zip
│   └── README.md
├── README.md
├── GOVERNANCE.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── CHANGELOG.md
├── LICENSE-APACHE
└── LICENSE-CC-BY
```

---

## 2. Tech stack lockdown (every dep pinned with rationale)

### 2.1 Rust workspace dependencies (top of `Cargo.toml`)
```toml
[workspace.dependencies]
# crypto
ed25519-dalek = "2.1"            # Ed25519 signing, audited, no_std-friendly
sha2 = "0.10"                    # SHA-256 (CT-style)
rs_merkle = "1.5"                # Merkle tree + proofs
zeroize = { version = "1.8", features = ["derive"] }

# data
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_canonical_json = "1"       # canonical JSON for hashing
cloudevents-sdk = "0.7"          # CNCF CloudEvents v1.0
jsonschema = "0.18"              # validate Decision Schema
uuid = { version = "1.10", features = ["v7", "serde"] }
time = { version = "0.3", features = ["serde", "macros"] }

# storage
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "macros", "time", "uuid", "json"] }

# runtime
tokio = { version = "1.40", features = ["macros", "rt-multi-thread", "fs"] }
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# CLI
clap = { version = "4.5", features = ["derive"] }
zip = "2"                        # regulator bundle packing

# testing
testcontainers = "0.21"
testcontainers-modules = { version = "0.9", features = ["postgres"] }
rapid = "0.6"                    # property-based testing
proptest = "1"                   # backup property framework
criterion = "0.5"                # benchmarks
insta = "1"                      # snapshot tests
```

### 2.2 Python SDK dependencies (`pyproject.toml`)
```toml
[project]
name = "attestly"
requires-python = ">=3.11"
dependencies = [
  "cloudevents>=1.10",
  "psycopg[binary]>=3.2",
  "cryptography>=43",            # only for verifier-side ed25519
  "click>=8.1",
]

[project.optional-dependencies]
dev = [
  "pytest>=8",
  "pytest-asyncio>=0.24",
  "pytest-cov>=5",
  "hypothesis>=6",
  "ruff>=0.6",
  "mypy>=1.11",
  "testcontainers[postgres]>=4",
]
adapters = [
  "anthropic>=0.40",             # Phase 1
  "openai>=1.50",                # Phase 1
  "langchain-core>=0.3",         # Phase 1
]

[build-system]
requires = ["maturin>=1.7"]
build-backend = "maturin"
```

### 2.3 Database
- Postgres 14+ (16 for dev). No extensions required for MVP (`pgcrypto` optional).
- Append-only enforced via REVOKE + trigger; documented in migration comments.

### 2.4 Tooling
- `cargo` + `cargo-nextest` for tests, `cargo-deny` for licence/advisory enforcement, `cargo-audit` weekly cron, `cargo-fuzz` in Phase 2.
- `uv` for Python env (faster than pip), `maturin` for PyO3 build.
- `act` for local CI debugging.
- `process-compose` or `docker compose` for local Postgres.

---

## 3. CI/CD pipeline (commit at end of Day 1)

### 3.1 `.github/workflows/ci.yml`
Triggers: every push to any branch, every PR.

Jobs:
- **lint-rust**: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo deny check`.
- **lint-python**: `ruff check`, `ruff format --check`, `mypy --strict sdk-python/attestly`.
- **lint-spec**: validate every `spec/*.json` against JSON Schema 2020-12 meta-schema.
- **test-rust** (matrix: linux/macos/windows × stable/beta):
  - `cargo nextest run --workspace`
  - Property tests get 5000 iterations on PRs, 20000 on main.
  - Integration tests spin up Postgres via testcontainers.
- **test-python** (matrix: linux/macos/windows × 3.11/3.12):
  - `maturin develop`
  - `pytest --cov=attestly --cov-fail-under=85`
- **build-wheels**: only on PRs touching `crates/attestly-pyo3/` or `sdk-python/`. Builds via `maturin build --release` on each OS, uploads as artifact.
- **bench-smoke**: on PRs touching `crates/attestly-core/src/{ledger,merkle}.rs`, run `cargo bench --bench append_throughput -- --quick`, compare against committed baseline, fail if regression > 20%.

Required checks for merge: lint-rust, lint-python, test-rust (linux+macos+windows on stable), test-python (linux+macos on 3.12).

### 3.2 `.github/workflows/release.yml`
Triggers: tag push matching `v*`.
- Build wheels on linux/macos/windows for Python 3.11+3.12.
- Build CLI binaries (cargo build --release) for all three OSes.
- Publish to crates.io (token in secrets).
- Publish to PyPI via trusted publishing (OIDC; no token).
- Create GitHub release with binaries attached + auto-generated changelog from conventional commits.

### 3.3 `.github/workflows/spec-publish.yml`
Triggers: push to main touching `spec/`.
- Renders markdown + JSON Schema browsers via Spectaql or Stoplight.
- Deploys to `spec.attestly.dev` via Cloudflare Pages.

---

## 4. Test strategy (6 layers)

| Layer | Tool | Target | When |
|---|---|---|---|
| Unit | `cargo test` / `pytest` | Each function | Every commit |
| Integration | testcontainers Postgres | Full ledger + verifier round-trip | Every commit |
| Property | `rapid` (Rust), `hypothesis` (Python) | Invariants: append-only, root determinism, proof correctness | Every commit (5k iters PR, 20k main) |
| Snapshot | `insta`, `pytest-regressions` | Wire format stability across versions | Every commit |
| Fuzz | `cargo-fuzz` | Bundle parser, checkpoint deserializer, CLI arg parser | Nightly (Phase 2+) |
| Benchmark | `criterion` | Perf budgets (§6) | Smoke on PR, full on main |

### 4.1 Key property invariants (must hold from Day 3)
- **Append-only**: for any sequence of N appends, then any K random UPDATE attempts, all UPDATEs fail with the expected error.
- **Root determinism**: given the same N leaves in the same order, `compute_root()` returns the same hash on two independent instances.
- **Inclusion proof correctness**: for any 0 ≤ i < N, `inclusion_proof(i)` verifies against `compute_root()`.
- **Inclusion proof tamper detection**: for any 0 ≤ i < N, mutating any byte of leaf[i] causes `verify(proof[i], root)` to fail.
- **Consistency proof (Phase 1)**: between tree sizes M < N, the consistency proof verifies and would detect a non-extending log.

### 4.2 Cross-language snapshot tests (Phase 1+)
- A canonical set of CloudEvents in `corpus/`.
- Python SDK produces a checkpoint; Rust core produces a checkpoint over the same events; their roots must be byte-identical.
- Snapshot file `corpus/snapshots/checkpoint-v0.1.json` is the cross-language oracle.

---

## 5. Spec artifacts (must ship with proposal as drafts; v0.1 published end of Phase 1 W4)

### 5.1 Decision Schema v0.1 (`spec/decision-schema-v0.1.md` + `.json`)
A CloudEvents v1.0 envelope with Attestly-specific extension attributes:

```json
{
  "specversion": "1.0",
  "id": "01J5...uuid7",
  "source": "did:web:operator.example/ai-system/credit-scorer",
  "type": "ai.attestly.decision.v1",
  "time": "2026-05-21T15:00:00Z",
  "datacontenttype": "application/json",
  "attestlyschemaversion": "0.1",
  "attestlydecisioncategory": "credit_score",   // Annex III category
  "attestlymodelid": "model-v3.4.1",
  "attestlysubjectref": "sha256:...",            // hashed pseudonymous subject id
  "attestlysensitivity": "private",              // never published in clear
  "data": {
    "decision": "denied",
    "score": 0.41,
    "factors": ["debt_ratio", "history_length"],
    "explanation_ref": "expl-49281"
  }
}
```

Required fields: `id`, `source`, `type`, `time`, `attestlyschemaversion`, `attestlydecisioncategory`. Optional: everything else.

### 5.2 Checkpoint format v0.1 (`spec/checkpoint-format-v0.1.md` + `.json`)
```json
{
  "version": "attestly-checkpoint/v0.1",
  "origin": "did:web:operator.example",
  "tree_size": 1234,
  "root_hash": "base64url:abc...",
  "timestamp": "2026-05-21T15:00:00Z",
  "signature": {
    "alg": "Ed25519",
    "key_id": "did:web:operator.example#ops-2026-05",
    "value": "base64url:xyz..."
  }
}
```

### 5.3 Verifier protocol v0.1 (`spec/verifier-protocol-v0.1.md`)
Plain markdown specifying:
- Bundle ZIP layout (decision.json, inclusion-proof.json, checkpoint-ref.txt, README.md)
- Verify algorithm (canonicalize → hash → verify inclusion → verify checkpoint signature → check origin DID)
- Exit codes (0 = valid, 1 = tampered, 2 = malformed bundle, 3 = network error fetching checkpoint, 4 = unknown DID)

### 5.4 Article 12 mapping (`spec/article12-mapping.md`)
Table mapping each sub-requirement of EU AI Act Article 12 to a concrete Attestly feature with code line references. Drafted Day 6, polished W10.

---

## 6. Performance budgets (publish in docs from W11)

| Operation | Median | p99 | Workload |
|---|---|---|---|
| `append()` (single event) | ≤ 2 ms | ≤ 5 ms | 10kB payload, Ed25519 sign, Postgres insert |
| `compute_root()` | ≤ 10 ms | ≤ 30 ms | 10,000 leaves |
| `inclusion_proof()` | ≤ 1 ms | ≤ 3 ms | tree of 10,000 |
| `verify_inclusion()` | ≤ 0.5 ms | ≤ 1 ms | proof of depth ~14 |
| `export --for-regulator` | ≤ 100 ms | ≤ 300 ms | single decision, includes DB read + ZIP write |
| Cold-start CLI | ≤ 50 ms | ≤ 150 ms | linux x86_64 |

Benchmarks live in `crates/attestly-core/benches/`. Baselines committed; CI flags >20% regression.

---

# PHASE 0 — Days 2–5 (MVP demo)

---

## 7. Day 2 (Fri 2026-05-23) — Core ingest

### Task 0.2.1 — Postgres schema + migration
**File**: `crates/attestly-core/migrations/0001_audit_log.sql`

```sql
-- Append-only event ledger. Enforce via REVOKE on app role + trigger guard.

CREATE TABLE IF NOT EXISTS audit_log (
    seq          BIGSERIAL PRIMARY KEY,
    system_did   TEXT      NOT NULL,
    event_id     UUID      NOT NULL UNIQUE,
    event_type   TEXT      NOT NULL,
    category     TEXT      NOT NULL,
    payload      JSONB     NOT NULL,
    payload_hash BYTEA     NOT NULL,
    prev_hash    BYTEA,
    sig          BYTEA     NOT NULL,
    signed_by    TEXT      NOT NULL,
    ts           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_system_did_ts ON audit_log (system_did, ts);
CREATE INDEX idx_audit_log_category ON audit_log (category);

-- Append-only enforcement: any UPDATE / DELETE raises.
CREATE OR REPLACE FUNCTION audit_log_append_only()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  RAISE EXCEPTION 'audit_log is append-only; UPDATE/DELETE forbidden';
END;
$$;

CREATE TRIGGER audit_log_no_update BEFORE UPDATE ON audit_log
  FOR EACH ROW EXECUTE FUNCTION audit_log_append_only();
CREATE TRIGGER audit_log_no_delete BEFORE DELETE ON audit_log
  FOR EACH ROW EXECUTE FUNCTION audit_log_append_only();

-- Operator role: explicit application user with INSERT only.
-- Setup script (not in migration): REVOKE UPDATE, DELETE, TRUNCATE ON audit_log FROM attestly_app;
```

**DoD**: migration runs on Postgres 14, 15, 16 via testcontainers (`sqlx migrate run`).

### Task 0.2.2 — Identity module
**File**: `crates/attestly-core/src/identity.rs`

```rust
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct SystemKey {
    signing: SigningKey,
}

impl SystemKey {
    pub fn generate() -> Self { /* OsRng */ }
    pub fn from_bytes(seed: [u8; 32]) -> Self { /* */ }
    pub fn public(&self) -> VerifyingKey { self.signing.verifying_key() }
    pub fn sign(&self, msg: &[u8]) -> Signature { self.signing.sign(msg) }
}

/// did:web identity for the operator org.
#[derive(Clone, Debug)]
pub struct OrgIdentity {
    pub did: String,                  // e.g. "did:web:operator.example"
    pub key_id: String,               // e.g. "did:web:operator.example#ops-2026-05"
    pub verifying: VerifyingKey,
}

impl OrgIdentity {
    /// Build a did:web DID Document for serving at /.well-known/did.json
    pub fn did_document(&self) -> serde_json::Value { /* */ }
}
```

**Tests**:
- `key_roundtrip_signs_and_verifies`
- `did_document_validates_against_did_core_schema`

### Task 0.2.3 — Event module + canonical hashing
**File**: `crates/attestly-core/src/event.rs`

```rust
use cloudevents::Event;
use sha2::{Sha256, Digest};

pub struct DecisionEvent {
    inner: Event,                     // CloudEvent
}

impl DecisionEvent {
    pub fn from_cloudevent(e: Event) -> Result<Self, ValidationError> {
        // Validate required Attestly extension attrs
        // Validate `data` against Decision Schema v0.1 JSON Schema
    }

    /// Canonical SHA-256 hash for ledger appending.
    /// Canonicalisation: JCS (RFC 8785).
    pub fn canonical_hash(&self) -> [u8; 32] {
        let canonical = serde_canonical_json::to_string(&self.inner).unwrap();
        Sha256::digest(canonical.as_bytes()).into()
    }
}
```

**Tests**:
- `validate_rejects_missing_attestly_attrs`
- `validate_rejects_unknown_category`
- `canonical_hash_stable_across_field_ordering` (property test: shuffle JSON keys, hash invariant)

### Task 0.2.4 — Ledger append
**File**: `crates/attestly-core/src/ledger.rs`

```rust
pub struct Ledger {
    pool: sqlx::PgPool,
    system_key: SystemKey,
    system_did: String,
}

pub struct AppendReceipt {
    pub seq: i64,
    pub event_id: uuid::Uuid,
    pub payload_hash: [u8; 32],
    pub sig: Vec<u8>,
}

impl Ledger {
    pub async fn append(&self, event: DecisionEvent) -> Result<AppendReceipt, AttestlyError> {
        let hash = event.canonical_hash();
        let sig = self.system_key.sign(&hash).to_bytes().to_vec();

        let prev_hash = sqlx::query_scalar!(
            "SELECT payload_hash FROM audit_log ORDER BY seq DESC LIMIT 1"
        ).fetch_optional(&self.pool).await?;

        let row = sqlx::query!(
            "INSERT INTO audit_log
               (system_did, event_id, event_type, category, payload, payload_hash, prev_hash, sig, signed_by)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
             RETURNING seq, event_id",
            self.system_did,
            event.id(),
            event.event_type(),
            event.category(),
            event.payload_json(),
            &hash[..],
            prev_hash.as_deref(),
            &sig,
            self.system_did,
        ).fetch_one(&self.pool).await?;

        Ok(AppendReceipt { seq: row.seq, event_id: row.event_id, payload_hash: hash, sig })
    }

    pub async fn read(&self, seq: i64) -> Result<Option<StoredEvent>, AttestlyError> { /* */ }
    pub async fn tree_size(&self) -> Result<u64, AttestlyError> { /* */ }
}
```

**Tests** (`crates/attestly-core/tests/append.rs`):
- `append_persists_event_and_returns_receipt`
- `append_records_prev_hash_chain`
- `update_attempt_raises_append_only_exception`
- `delete_attempt_raises_append_only_exception`

### Task 0.2.5 — PyO3 binding scaffolding
**File**: `crates/attestly-pyo3/src/lib.rs`

```rust
use pyo3::prelude::*;

#[pymodule]
fn _attestly(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyLedger>()?;
    Ok(())
}

#[pyclass(name = "Ledger")]
struct PyLedger { inner: attestly_core::Ledger }

#[pymethods]
impl PyLedger {
    #[new] fn new(pg_url: String, system_key_b64: String, system_did: String) -> PyResult<Self> { /* */ }
    fn append(&self, cloudevent: &Bound<PyDict>) -> PyResult<PyAppendReceipt> { /* */ }
}
```

**Acceptance**: `maturin develop` builds; `python -c "from attestly import Ledger; print(Ledger)"` works.

**Day 2 exit criteria**: `cargo nextest run -p attestly-core` green. PyO3 module imports. Integration test "append 10 events, query back, all present" passes.

---

## 8. Day 3 (Sat 2026-05-24) — Merkle commitments + signed checkpoint

### Task 0.3.1 — Merkle module
**File**: `crates/attestly-core/src/merkle.rs`

```rust
use rs_merkle::{MerkleTree, algorithms::Sha256 as MerkleSha256, Hasher};

pub fn build_tree(leaves: &[[u8; 32]]) -> MerkleTree<MerkleSha256> {
    MerkleTree::<MerkleSha256>::from_leaves(leaves)
}

pub fn root(tree: &MerkleTree<MerkleSha256>) -> Option<[u8; 32]> { tree.root() }

pub fn inclusion_proof(tree: &MerkleTree<MerkleSha256>, leaf_index: usize) -> InclusionProof {
    let proof = tree.proof(&[leaf_index]);
    InclusionProof {
        leaf_index,
        siblings: proof.proof_hashes().to_vec(),
    }
}

pub fn verify_inclusion(
    proof: &InclusionProof,
    leaf: [u8; 32],
    root: [u8; 32],
    tree_size: usize,
) -> bool {
    rs_merkle::MerkleProof::<MerkleSha256>::new(proof.siblings.clone())
        .verify(root, &[proof.leaf_index], &[leaf], tree_size)
}
```

**Property tests** (`tests/property.rs` via `rapid`):
- `root_is_deterministic`: shuffle order → different roots; same order → same root.
- `every_leaf_has_valid_proof`
- `tampered_leaf_invalidates_proof`
- `wrong_root_invalidates_proof`

### Task 0.3.2 — Checkpoint module
**File**: `crates/attestly-core/src/checkpoint.rs`

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct SignedCheckpoint {
    pub version: String,            // "attestly-checkpoint/v0.1"
    pub origin: String,             // did:web
    pub tree_size: u64,
    pub root_hash: String,          // base64url
    pub timestamp: time::OffsetDateTime,
    pub signature: CheckpointSignature,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckpointSignature {
    pub alg: String,                // "Ed25519"
    pub key_id: String,             // did:web#key-id
    pub value: String,              // base64url
}

impl SignedCheckpoint {
    pub fn build(
        origin: &OrgIdentity,
        org_key: &SystemKey,
        tree_size: u64,
        root_hash: [u8; 32],
    ) -> Self { /* sign canonical form */ }

    pub fn signed_bytes(&self) -> Vec<u8> {
        // Canonical pre-image: version|origin|tree_size|root|timestamp
        // Sign exactly this.
    }

    pub fn verify(&self, key: &VerifyingKey) -> Result<(), VerifyError> { /* */ }
}
```

**Tests**:
- `checkpoint_signs_and_verifies`
- `tampered_root_invalidates_checkpoint`
- `wrong_key_invalidates_checkpoint`

### Task 0.3.3 — Ledger checkpoint API
Extend `crates/attestly-core/src/ledger.rs`:

```rust
impl Ledger {
    pub async fn current_root(&self) -> Result<[u8; 32], AttestlyError> {
        let hashes: Vec<[u8; 32]> = sqlx::query_scalar!(
            "SELECT payload_hash FROM audit_log ORDER BY seq"
        ).fetch_all(&self.pool).await?
         .into_iter().map(|b| b.try_into().unwrap()).collect();
        Ok(merkle::root(&merkle::build_tree(&hashes)).unwrap_or([0; 32]))
    }

    pub async fn sign_checkpoint(
        &self,
        org: &OrgIdentity,
        org_key: &SystemKey,
    ) -> Result<SignedCheckpoint, AttestlyError> {
        let size = self.tree_size().await?;
        let root = self.current_root().await?;
        Ok(SignedCheckpoint::build(org, org_key, size, root))
    }

    pub async fn inclusion_proof_for_seq(&self, seq: i64) -> Result<InclusionProof, AttestlyError> { /* */ }
}
```

### Task 0.3.4 — `attestly publish` CLI subcommand
**File**: `crates/attestly-cli/src/cmd/publish.rs`

```
attestly publish-checkpoint --target file:///var/lib/attestly/checkpoints/
attestly publish-checkpoint --target https://logs.attestly.dev/upload  (Phase 2)
```

Writes `checkpoint-<unixts>.json` to the target. For MVP, file:// only. HTTP target lands Phase 2 W8.

**Day 3 exit criteria**: `attestly publish-checkpoint --target file:///tmp/attestly-logs/` produces a valid JSON file that re-parses and verifies. Property tests pass at 5000 iterations.

---

## 9. Day 4 (Sun 2026-05-25) — Verifier CLI + regulator-export workflow

### Task 0.4.1 — `attestly export --for-regulator`
**File**: `crates/attestly-cli/src/cmd/export.rs`

CLI:
```
attestly export --decision-id <seq> --for-regulator --out regulator-<seq>.zip
```

Bundle layout:
```
regulator-<seq>.zip
├── README.md             (1pp human-readable verification instructions)
├── decision.json         (the canonical CloudEvent payload)
├── inclusion-proof.json  (leaf index, siblings, tree_size)
├── checkpoint-ref.json   (origin DID, key_id, tree_size, root, signature, fetch_url)
└── did-document.json     (snapshot of operator's did:web document)
```

**Acceptance**: a fresh ZIP unzips on Linux, macOS, Windows; structure validates against a snapshot test in `tests/cli_smoke.rs`.

### Task 0.4.2 — `attestly-verifier` crate (no DB dependency)
**File**: `crates/attestly-verifier/src/lib.rs`

```rust
pub enum VerifyResult {
    Valid,
    Tampered { reason: String, expected: String, actual: String },
    Malformed { reason: String },
    UnknownOrigin,
}

pub fn verify_bundle(bundle_zip_path: &Path) -> VerifyResult { /* */ }
pub fn verify_inclusion_only(
    leaf_hash: [u8; 32],
    proof: &InclusionProof,
    checkpoint: &SignedCheckpoint,
    key: &VerifyingKey,
) -> VerifyResult { /* */ }
```

The verifier crate **must compile without any database, network, or filesystem-mutation dependencies**. This is the "regulator runs it locally" promise.

**Tests** (`tests/golden.rs`):
- Verify a known-good corpus bundle: PASS.
- Verify a bundle where `decision.json` was edited by 1 byte: TAMPERED.
- Verify a bundle where `inclusion-proof.json` is empty: MALFORMED.
- Verify a bundle whose checkpoint signature is wrong: TAMPERED.

### Task 0.4.3 — `attestly verify --bundle` CLI subcommand
**File**: `crates/attestly-cli/src/cmd/verify.rs`

```
attestly verify --bundle regulator-23.zip
  → exit 0 + stdout "OK: decision seq=23 matches published commitment"
  → exit 1 + stderr "FAIL: decision seq=23 no longer matches checkpoint root"
```

### Task 0.4.4 — End-to-end smoke test
**File**: `crates/attestly-cli/tests/e2e.rs`

```rust
#[test]
fn e2e_append_publish_export_verify_tamper_detect() {
    // 1. Spin up Postgres via testcontainers
    // 2. Run `attestly append` 50 times with synthetic credit decisions
    // 3. Run `attestly publish-checkpoint --target file:///tmp/...`
    // 4. Run `attestly export --decision-id 23 --for-regulator --out /tmp/r-23.zip`
    // 5. Run `attestly verify --bundle /tmp/r-23.zip` → assert exit 0
    // 6. Run `psql -c "UPDATE audit_log SET payload = ... WHERE seq = 23"`
    //    (in test mode the trigger is dropped — simulate root-level tamper)
    // 7. Run `attestly export --decision-id 23 --for-regulator --out /tmp/r-23-tampered.zip`
    // 8. Run `attestly verify --bundle /tmp/r-23-tampered.zip` → assert exit 1
}
```

**Day 4 exit criteria**: the e2e smoke test passes on linux + macOS + Windows. This is the demo script in code form.

---

## 10. Day 5 (Mon 2026-05-26) — Demo + v0.1.0 cut

### Task 0.5.1 — `examples/credit_score_decision.py`
Synthetic credit-scoring AI emitting 50 decisions over varying inputs. Calls Attestly Python SDK. ~80 lines.

### Task 0.5.2 — Demo screencast (1080p, 90s, narrated)
Script:
1. (0:00) Title card: "Attestly — open verification for EU AI Act Article 12"
2. (0:05) "A citizen disputes a credit denial. The regulator asks for evidence on decision 23."
3. (0:10) `attestly export --decision-id 23 --for-regulator` → bundle created.
4. (0:20) `attestly verify --bundle regulator-23.zip` → OK.
5. (0:30) "The operator quietly tampers with their database after the fact."
6. (0:35) `psql -c "UPDATE audit_log ..."` → 1 row affected.
7. (0:45) Re-run `export` + `verify` → FAIL with reason.
8. (1:00) "The log is intact. The operator's claim is not. That's Article 12 evidence integrity."
9. (1:10) "Open source, framework-agnostic, runs on any Postgres. github.com/attestly"

Tool: OBS Studio + cursor highlighter. Record twice; pick the better take. Upload to YouTube (unlisted) + embed in STF proposal and EIC Step 1 short application.

### Task 0.5.3 — README + repo polish
README structure (top-to-bottom, 1 page):
1. One-sentence pitch (the moat sentence)
2. 90s demo embed
3. `pip install attestly` + 10-line Python "hello world"
4. Architecture diagram (link to spec)
5. Three links: spec, docs, governance
6. "Funding from Sovereign Tech Fund (pending)" footer

### Task 0.5.4 — `v0.1.0` tag
Once demo works:
- `git tag v0.1.0 -m "v0.1.0 — MVP demo"`
- `git push origin v0.1.0` → triggers release workflow → publishes to crates.io + PyPI

**Day 5 exit criteria**: a fresh user can `pip install attestly`, run the credit-score example, produce and verify a bundle, and watch it fail after a tamper — all within 10 minutes of landing on the README. If that doesn't work, apply the Day 5 fallback plan (SPRINT-PLAN.md §8).

---

# PHASE 1 — Weeks 1–6 (Evaluation period, not grant-dependent)

Stories are user-story-shaped with explicit acceptance criteria.

---

## 11. Week 1 — Polish core + ship v0.2

### Story P1.W1.1 — Operational hardening
**As an** operator running Attestly in dev
**I want** clear, structured error messages and observability
**So that** I can debug integration issues without reading source

Acceptance:
- All public functions return `Result<_, AttestlyError>` with `thiserror`-defined variants.
- `tracing` instrumentation on all public APIs with span attributes (system_did, event_id).
- A `tracing-subscriber` JSON formatter wired into the CLI behind `RUST_LOG`.

### Story P1.W1.2 — Performance baseline + benchmarks
Acceptance:
- Criterion benchmarks for `append`, `current_root` (1k, 10k, 100k leaves), `inclusion_proof`, `verify_inclusion`.
- Baselines committed to `benches/baselines/`.
- CI fails on >20% regression.
- Numbers published in `docs/benchmarks.md`.

### Story P1.W1.3 — v0.2.0 release
Acceptance: tag, release workflow green, crates.io + PyPI updated, CHANGELOG.md entry.

---

## 12. Week 2 — Python SDK polish + adapter foundation

### Story P1.W2.1 — High-level Python wrapper
**File**: `sdk-python/attestly/ledger.py`

Acceptance:
- `attestly.Ledger.from_env()` constructs from `ATTESTLY_*` env vars.
- `attestly.Ledger.append_decision(category, model_id, payload)` — convenience over raw CloudEvents.
- Context-manager support: `with attestly.Ledger.from_env() as ledger: ...`.
- Async variant: `attestly.AsyncLedger`.

### Story P1.W2.2 — Adapter framework
**File**: `sdk-python/attestly/adapters/__init__.py`

Acceptance:
- Define an `AuditAdapter` protocol (Python `typing.Protocol`).
- Two reference implementations: `CallableAdapter` (manual `.log_decision()`) and `MiddlewareAdapter` (for framework callbacks).
- Documented integration pattern for adding new adapters.

### Story P1.W2.3 — pytest fixture library
**File**: `sdk-python/attestly/testing.py`

Acceptance: a `@pytest.fixture def attestly_ledger()` that spins up testcontainers Postgres + initialised Ledger, usable by downstream projects' tests.

---

## 13. Weeks 3–4 — Spec v0.1 publication

### Story P1.W3.1 — Decision Schema v0.1 finalised
Acceptance:
- `spec/decision-schema-v0.1.json` validates against JSON Schema 2020-12 meta-schema.
- 10 example documents in `spec/examples/decision/`, each annotated.
- Markdown narrative spec (`spec/decision-schema-v0.1.md`) 5-10 pages.
- CI job that validates every example against the schema.

### Story P1.W3.2 — Checkpoint format v0.1 finalised
Same shape as W3.1 for checkpoints.

### Story P1.W4.1 — Verifier protocol v0.1 spec
Acceptance:
- Reference verifier in `attestly-verifier` crate matches the spec exactly.
- A "verifier conformance" CLI: `attestly-conformance test --verifier <path>` runs corpus bundles through any verifier and checks results.

### Story P1.W4.2 — Spec hosting
Acceptance: `spec.attestly.dev` live, deploys on every push to main touching `spec/`.

### Story P1.W4.3 — Article 12 mapping doc draft
**File**: `spec/article12-mapping.md`. Each Article 12 sub-paragraph mapped to a concrete Attestly feature with line references. Reviewed by an EU AI Act-literate reader (find one in network).

---

## 14. Weeks 5–6 — Conforme pilot integration + second pilot outreach

### Story P1.W5.1 — Conforme NRUA decisions logged to Attestly
Acceptance:
- Inside Conforme's NRUA wizard codebase, after each regulatory decision (provider classification, rule resolution, license-check outcome), emit a CloudEvent to a local Attestly ledger.
- Conforme produces a daily checkpoint published to `logs.conforme.info/attestly/`.
- Integration documented in `docs/integrations/conforme.md`.

### Story P1.W6.1 — Worked example: employment screening
**File**: `examples/employment_screening.py`. ~150 lines. Demonstrates a multi-stage screening pipeline with decisions at each stage.

### Story P1.W6.2 — Second pilot user identified + outreach
Not coding — but required artifact: a public statement of intent from a non-Conforme pilot user. Required for STF evaluator credibility on "not a Conforme spinoff".

### Story P1.W6.3 — Phase 1 retro + Phase 2 plan adjustment
Half-day retro on what shipped vs Phase 1 plan. Adjust Phase 2 based on STF and EIC Step 1 decisions if known by this point. Trigger Lda incorporation if EIC Step 1 returned GO (required for ANI Voucher Deep Tech €60k+€10k follow-on).

---

# PHASE 2 — Weeks 7–18 (Funded build, conditional on STF approval)

---

## 15. Weeks 7–8 — TypeScript SDK

### Story P2.W7.1 — napi-rs setup
Acceptance:
- `sdk-typescript/` builds via napi-rs targeting linux-x64, macOS-arm64, windows-x64.
- `@attestly/sdk` published to npm under the `attestly` org.
- TypeScript types exported with `.d.ts`.

### Story P2.W7.2 — TS API parity with Python
Acceptance:
- `Ledger`, `Org`, `SignedCheckpoint`, `Bundle` classes mirror Python signatures.
- README has TS quickstart matching Python quickstart.

### Story P2.W8.1 — Cross-language snapshot tests
Acceptance:
- `corpus/snapshots/checkpoint-v0.1.json` is the oracle.
- CI runs Rust + Python + TS producing checkpoints over the same corpus, asserts byte-identical roots.

### Story P2.W8.2 — TS adapter foundations
Acceptance: skeleton TS adapters for Anthropic Agent SDK, OpenAI Agents SDK, Vercel AI SDK. Full impl in W14.

---

## 16. Week 9 — Production transparency-log integration (Tessera)

### Story P2.W9.1 — Tessera Postgres-backed personality
Acceptance:
- Replace MVP file-based STH publication with Tessera's PostgreSQL backend driver.
- `logs.attestly.dev` live: serves checkpoints + inclusion proofs over HTTPS.
- Tessera's GCP/AWS backends documented as alternatives but not required.

### Story P2.W9.2 — Witness cosigning (optional, stretch)
Acceptance:
- A simple witness server (`attestly-witness` crate) that countersigns Attestly checkpoints, providing the "two-party gossip" property from Certificate Transparency.
- Conforme runs the first witness. A second witness target identified.

---

## 17. Week 10 — Web verifier (WASM)

### Story P2.W10.1 — `attestly-verifier` compiled to WASM
Acceptance:
- `cargo build --target wasm32-unknown-unknown -p attestly-verifier`.
- A 200kB-or-less WASM artifact loaded by a vanilla HTML page (no JS framework).
- User drags a `regulator-N.zip` into the page; verifier runs in browser; verdict displayed.

### Story P2.W10.2 — `verify.attestly.dev` deployed
Acceptance: hosted on Cloudflare Pages; zero JS dependencies; works offline once loaded.

---

## 18. Week 11 — Article 12 export pack

### Story P2.W11.1 — `attestly export --article-12`
Acceptance:
- Produces a regulator-ready bundle: log entries in range, all checkpoints covering the range, all inclusion proofs, organisation's did:web document, README mapping to Article 12 sub-requirements.
- Bundle structure documented in `spec/article12-bundle-v0.1.md`.

### Story P2.W11.2 — Compliance mapping docs polished
Acceptance:
- `docs/eu-ai-act-article-12-mapping.md` reviewed by EU AI Act-literate reader.
- `docs/soc2-mapping.md` drafted (CC7.2 logging control mapping).
- `docs/gdpr-posture.md` explains commitments-not-payloads pattern in GDPR terms.

---

## 19. Week 12 — Docs site + 3 worked examples polished

### Story P2.W12.1 — `docs.attestly.dev` live
Acceptance:
- MkDocs Material site with sections: Quickstart · Concepts · Spec · Integrations · Compliance · FAQ · Benchmarks.
- Search works.
- All examples runnable from quickstart in <10 minutes by a new user.

### Story P2.W12.2 — Three polished examples
1. Credit scoring: `examples/credit_scoring/`
2. Employment screening: `examples/employment_screening/`
3. AGT bridge skeleton (full implementation in W14): `examples/agt_bridge/`

### Story P2.W12.3 — User-testing pass
Acceptance: 10 friendly testers walk through the quickstart; ≥7 finish without help; collect feedback issues filed.

---

## 20. Week 13 — Security review prep

### Story P2.W13.1 — Threat model document
**File**: `docs/threat-model.md` ~5 pages.
- Assets: integrity of the log, integrity of checkpoints, key material, operator identity.
- Adversaries: malicious operator, compromised CI, malicious witness, MITM on log fetch.
- What Attestly defends: tamper-after-publication, undetected log truncation, key forgery (within ed25519 assumptions).
- What it doesn't: pre-publication manipulation (operator controls the source); collusion of operator + all witnesses; offline-attack on the checkpoint signing key.

### Story P2.W13.2 — Static analysis pass
Acceptance:
- `cargo audit` clean.
- `cargo deny` clean for licences + advisories.
- `pip-audit` clean.
- `npm audit` clean.
- `cargo-fuzz` harnesses for: bundle ZIP parser, checkpoint JSON deserializer, CLI arg parser.

### Story P2.W13.3 — External review engagement
Acceptance: contract signed with Radically Open Security (or equivalent) for an independent security review. STF typically funds review separately on top of grant; budget defensively (€5k line in §13 of SPRINT-PLAN.md). Audit kicks off W13 → report by W17.

---

## 21. Week 14 — AGT bridge adapter

### Story P2.W14.1 — `attestly-bridge-agt` adapter
Acceptance:
- Subscribes to a Microsoft AGT CloudEvents stream (file sink, Kafka, or webhook).
- Maps AGT event types to Attestly Decision Schema where possible.
- Documents the mapping in `docs/integrations/microsoft-agt.md`.
- Worked example in `examples/agt_bridge/`.

### Story P2.W14.2 — Generic CloudEvents subscriber
Acceptance: a generic `attestly subscribe --source cloudevents://...` CLI subcommand that ingests any CloudEvents source. Lets Attestly be a sink for *any* compliant emitter.

---

## 22. Weeks 15–16 — Standards engagement

### Story P2.W15.1 — Decision Schema submission to standards body
Acceptance:
- Submit Decision Schema v0.1 to either CNCF CloudEvents Working Group (as a profile) or W3C Community Group (as a draft spec). Decision documented in `docs/standards-strategy.md`.
- Initial acknowledgement received from chosen body.

### Story P2.W15.2 — W3C Agent Identity Registry Protocol CG participation
Acceptance:
- Register as a participant in the W3C Agent Identity Registry Protocol Community Group.
- Submit a contribution document arguing for compatibility between AIR Protocol identifiers and Attestly Decision Schema's `source` field.

### Story P2.W16.1 — IETF SCITT engagement
Acceptance: contribute to IETF SCITT (Supply Chain Integrity, Transparency, and Trust) WG — file a position document or Internet-Draft mapping Attestly's transparency-log model onto SCITT receipt patterns. Strong signal of standards-track ambition for the STF M3-M4 milestones.

### Story P2.W16.2 — Conference talk submissions
Acceptance:
- FOSDEM 2027 talk submission filed (Open Source AI / Security devroom).
- Internet Identity Workshop (IIW) Spring 2027 session proposal filed.
- RightsCon or Privacy Enhancing Technologies Symposium talk submission filed (civil-society audience).

---

## 23. Weeks 17–18 — v1.0 release + launch

### Story P2.W17.1 — v1.0.0 tag
Acceptance:
- All Phase 1 + Phase 2 stories closed or explicitly deferred to v1.1.
- Security audit report from Radically Open Security received; all blockers resolved.
- CHANGELOG.md complete; migration notes from v0.x.
- Tag `v1.0.0` → release workflow publishes crates.io + PyPI + npm + GitHub Release.

### Story P2.W17.2 — Documentation freeze
Acceptance: all `docs/` content reviewed; broken links removed; outdated examples updated; benchmark numbers refreshed.

### Story P2.W18.1 — Launch announcement
Acceptance:
- Blog post on `docs.attestly.dev/blog/` titled something like "EU AI Act Article 12 evidence integrity in a Rust crate".
- Post submitted to Hacker News, Lobsters, dev.to, r/MachineLearning, EU AI Act subreddit, Mastodon Fediverse.
- Press outreach to EU-policy-aware tech press (TechCrunch EU, Sifted, Politico EU Tech newsletter).

### Story P2.W18.2 — Retro + Phase 3 plan + NLnet Oct follow-on
Half-day retro. Write `docs/roadmap-v1.1.md`. Common Phase 3 candidates: selective-disclosure primitive, RFC 3161 timestamping, federated multi-org logs, EUDI Wallet binding for operator identity.

**Concurrent**: prepare NLnet NGI0 Commons application for 1 October cycle as follow-on funding (Phase 3 scope). By W18: Conforme NLnet decision known, Attestly STF decision known, Attestly v1.0 has shipped — strong returning-applicant case for NLnet.

---

## 24. Performance budgets revisited at v1.0

| Operation | v0.1 (Day 5) | v1.0 target |
|---|---|---|
| `append()` median | best-effort | ≤ 2 ms |
| `compute_root()` 10k leaves | best-effort | ≤ 10 ms |
| `verify_inclusion()` median | best-effort | ≤ 0.5 ms |
| End-to-end export+verify | best-effort | ≤ 500 ms |
| WASM verifier bundle size | n/a | ≤ 200 kB gzipped |
| Cold-start CLI | best-effort | ≤ 50 ms median, ≤ 150 ms p99 (linux x86_64) |

Numbers published in `docs/benchmarks.md` from W12 onward.

---

## 25. What you own vs what I own

### You own (decisions/actions only you can take)
| Item | Deadline | Why I can't do it |
|---|---|---|
| **GitHub org `attestly` registration** | Day 1 | Requires your account + creates a public commitment to the name |
| **Domain `attestly.dev` purchase** | Day 1 | Credit card transaction |
| **PyPI namespace `attestly` reservation** | Day 1 | Requires your PyPI account + email verification |
| **npm scope `@attestly/sdk` reservation** | Day 1 | Same |
| **STF applicant account at sovereign.tech/programs/fund** | Day 1 | Your identity + email |
| **EU Funding & Tenders Portal account (for EIC Step 1)** | Day 1 | Your identity + EU login (ECAS) |
| **Conforme bandwidth commit (60% for 10 days)** | Day 1 | Only you can promise it |
| **External reviewer identification (2 people in your network)** | Day 6 | I don't know your network |
| **Letter-of-support outreach** (optional for STF) | Day 7 (I draft, you send) | Cold emails from a known sender land; from me they don't |
| **Conforme backer letter** (optional for STF) | Day 7 (I draft, you sign) | Same |
| **STF + EIC Step 1 final edit + submit click** | Day 10 | You're the legal applicant |
| **Lda incorporation** | Post-EIC Step 1 GO (~6 weeks after Day 10) | Notarial + accountant work. Defer until GO to avoid €500+ sunk cost on NO. |
| **ANI Voucher Deep Tech filing** | After Lda + EIC Step 1 GO | Requires Lda + ANI portal account |

### I own (pure coding + drafting)
- All technical decisions: stack, schema, function signatures, CI, tests, perf budgets.
- All code in `crates/`, `sdk-python/`, `sdk-typescript/`.
- All spec markdown + JSON Schemas in `spec/`.
- All examples in `examples/`.
- CI workflows + release pipeline.
- Documentation site + content.
- Draft STF proposal text and EIC Step 1 short application (you edit + submit).
- Draft letters of support (you send under your name).
- Demo screencast script (we record together; you narrate or I do).
- Draft NLnet October-cycle follow-on proposal text in W18.

### Joint
- Naming overrides (you can veto Attestly until end of Day 1).
- Scope cuts under pressure (Day 5 fallback decision, etc.).
- Pilot user introductions (you identify, I do technical onboarding).
- Standards-body choice in W15 (CNCF vs W3C — depends on which has friendlier ingress for solo contributors).

---

## 26. Open technical questions (resolve before Day 3)

- [ ] **Canonical JSON form for hashing**: JCS (RFC 8785) or LD-Sign? Default to JCS unless DID community insists otherwise.
- [ ] **Witness model in v1.0**: include it or defer to v1.1? Default: include simple single-witness in W9, multi-witness in v1.1.
- [ ] **Checkpoint publication cadence**: per-event (slow), per-batch (need to define batch), or time-driven (every 60s)? Default: time-driven 60s with manual trigger override.
- [ ] **Bundle format versioning**: how do future v0.2 bundles handle v0.1 verifiers? Default: hard version field; verifier refuses unknown versions with clear error.
- [ ] **Standards-body venue (W15)**: CNCF CloudEvents WG (profile contribution) or W3C Community Group (draft spec)? Resolve by W14 after talking to both.

---

## 27. Reference URLs

See `SPRINT-PLAN.md` §17. Plus:
- [rs_merkle docs](https://docs.rs/rs_merkle/)
- [ed25519-dalek docs](https://docs.rs/ed25519-dalek/)
- [PyO3 user guide](https://pyo3.rs/)
- [napi-rs docs](https://napi.rs/)
- [sqlx docs](https://docs.rs/sqlx/)
- [testcontainers-rs](https://github.com/testcontainers/testcontainers-rs)
- [JCS RFC 8785](https://www.rfc-editor.org/rfc/rfc8785)
- [CloudEvents Rust SDK](https://github.com/cloudevents/sdk-rust)
- [IETF SCITT WG](https://datatracker.ietf.org/wg/scitt/about/)
- [W3C Agent Identity Registry Protocol CG](https://www.w3.org/community/agent-identity/)
- [CNCF CloudEvents WG](https://github.com/cncf/wg-serverless)

---

*End of coding sprint. Phase 0 is ~24 hours of focused coding. Phase 1 is 6 weeks of polish + spec + first pilot. Phase 2 is 12 weeks of v1.0 + standards engagement if STF-funded (€100k). All pure coding except for §25's "you own" items.*
