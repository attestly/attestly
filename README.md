# Attestly

> Open, framework-agnostic verification infrastructure for **EU AI Act Article 12** evidence.
>
> Records AI decision events in an append-only signed ledger, publishes Merkle-rooted Signed Tree Heads to a public log, and lets a regulator independently verify any single decision against the published commitment — without trusting the operator and without depending on any single commercial AI stack.

## Status

Pre-NGI / Sovereign Tech Fund submission. Working PoC + demo, ahead of full submission targeted for end-May 2026.

## Quickstart

```sh
# Build the CLI (~60s first time)
cargo build --release

# Run the end-to-end demo (bash)
bash examples/demo.sh

# or PowerShell on Windows
pwsh examples/demo.ps1
```

The demo runs the full pipeline: initialise keys + SQLite ledger, append 50 synthetic credit-scoring decisions, publish a signed checkpoint, export an evidence bundle for decision #23, verify it (PASS), tamper with the database bypassing the append-only trigger, re-export, and verify again (FAIL — with a precise hash mismatch report).

## What it does

```
                  ┌──────────────────────────────┐
                  │  Any high-risk AI system     │
                  │  (credit, hiring, medical…)  │
                  └──────────────┬───────────────┘
                                 │ CloudEvent
                                 ▼
                  ┌──────────────────────────────┐
                  │  Attestly SDK                │
                  │  - sign with system key      │
                  │  - append to local ledger    │
                  └──────────────┬───────────────┘
                                 │
                  ┌──────────────▼───────────────┐
                  │  Append-only audit_log       │
                  │  (operator-controlled)       │
                  └──────────────┬───────────────┘
                                 │ Merkle root + signature
                                 ▼   ── public boundary ──
                  ┌──────────────────────────────┐
                  │  Public transparency log     │
                  │  (commitments only, no PII)  │
                  └──────────────┬───────────────┘
                                 │ inclusion proof + bundle
                                 ▼
                  ┌──────────────────────────────┐
                  │  Regulator / citizen verifier│
                  │  `attestly verify`           │
                  └──────────────────────────────┘
```

## Why it matters

EU AI Act Article 12 mandates automatic logging by all high-risk AI systems from **2 August 2026**. The regulation requires that logs exist; it is silent on whether they are tamper-evident. Today, an operator can edit their own audit trail before a regulator inspects it.

Attestly publishes only cryptographic *commitments* (Merkle roots + signatures) to a public log. Decision payloads remain private under operator control — GDPR-compatible by construction. A regulator can verify any single decision by comparing the canonical hash of its exported payload against the publicly-committed leaf. If the operator alters the audit trail after publication, the verifier mathematically detects it.

Same pattern as Certificate Transparency for the web's PKI, or Sigstore Rekor for software supply chains, applied to AI decision evidence.

## Project layout

```
attestly/
├── Cargo.toml                # workspace
├── crates/
│   ├── attestly-core/        # ledger, merkle, checkpoint, identity, event
│   ├── attestly-cli/         # binary `attestly` (init, append, publish, export, verify)
│   └── attestly-verifier/    # standalone verifier (no database dependency)
├── examples/
│   ├── demo.sh               # bash end-to-end demo
│   └── demo.ps1              # PowerShell end-to-end demo
├── SPRINT-PLAN.md            # grant submission strategy + timeline
└── CODING-SPRINT.md          # task-level coding plan (Phase 0–2)
```

## Status of the demo

- ✅ Phase 0 Day 2 — Append-only SQLite ledger + Ed25519 signing
- ✅ Phase 0 Day 3 — Merkle tree + signed checkpoint
- ✅ Phase 0 Day 4 — Verifier CLI + regulator export bundle
- ✅ Phase 0 Day 5 — End-to-end tamper-detection demo

## Funding

Submission in flight to **Sovereign Tech Fund** (primary, €100,000) and EIC Accelerator Step 1 (secondary, triggers €60k ANI Voucher Deep Tech on GO). NLnet NGI0 Commons deferred to 1 October cycle pending Conforme decision.

## Licence

- Code: Apache-2.0
- Specification + docs: CC-BY-4.0
