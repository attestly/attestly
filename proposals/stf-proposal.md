# Sovereign Tech Fund Proposal — Attestly

> Open verification infrastructure for EU AI Act Article 12 evidence
>
> Submission target: rolling, **Sat 31 May 2026** (10-day pre-submission sprint).
> Funding ask: **€100,000** (Sovereign Tech Fund minimum €50k, no upper cap).
> Funder portal: https://www.sovereign.tech/programs/fund

---

## Project title

**Attestly — Open verification infrastructure for EU AI Act Article 12 evidence**

## Tagline (≤ 200 chars)

A drop-in tamper-evident audit log and public verifier for high-risk AI systems. Decision payloads stay private; cryptographic commitments go public. Any regulator can verify independently.

## Short summary (~ 150 words)

EU AI Act Article 12, enforceable from **2 August 2026**, mandates automatic logging by every high-risk AI system. The Act is silent on whether those logs can be trusted: today, an operator can edit their own audit trail before a regulator inspects it, and **the regulatory framework has no technical assurance layer for evidence admissibility**. Penalties for record-keeping failures reach €15M or 3% of global turnover.

Attestly is open digital base infrastructure that closes this gap. It records AI decision events in an append-only signed ledger, publishes Merkle-rooted Signed Tree Heads to a public transparency log, and provides a regulator-runnable verifier that requires no operator cooperation to validate any single decision. The pattern is Certificate Transparency applied to AI accountability — **public commitments, private payloads** (GDPR-compatible by construction). The result for the regulator is **independent audit defensibility**; for the operator, **compliance-burden reduction** (Article 12 evidence integrity ceases to be a per-system custom build).

Apache-2.0 + CC-BY-4.0 in perpetuity. No commercial dependency.

---

## Project description (long form, ~1500 words)

### The problem

The EU AI Act (Regulation EU 2024/1689) categorises a wide range of AI systems as "high-risk" under Annex III: credit scoring, hiring decisions, biometric identification, employment screening, access to essential services, law enforcement, migration management, and judicial decisions. From **2 August 2026**, every operator of such a system must maintain automatic logs over the system's full lifecycle (Article 12), retained for at least 6 months and accessible to market surveillance authorities (Article 26(6)). Operators that fail to comply face penalties up to **€15M or 3% of global turnover** (Article 99).

Article 12 is explicit about *what* must be logged (events sufficient to support post-market monitoring under Article 72) but **silent on integrity**. A determined operator with admin access to their own audit-log database can alter records after the fact. A regulator, presented with an audit log on demand, has no way to verify whether that log is the original record or a rewritten version. The legal framework is in place; the technical assurance is not.

This gap is not theoretical. The same gap exists in every high-stakes domain where a single party both produces and stores evidence about its own conduct — and it was solved decades ago by transparency logs. Certificate Transparency (RFC 6962) eliminated rogue TLS certificate issuance by forcing Certificate Authorities to publish a Merkle-rooted log of every certificate they issue, with public verifiability. Sigstore's Rekor applied the same pattern to software-supply-chain attestations. Microsoft's Agent Governance Toolkit (April 2026) is the first widely-marketed application of tamper-evident audit logs to AI systems — but only to agentic systems within Microsoft's own framework stack, with in-process verification.

Most high-risk AI under Annex III is **not agentic** (credit scoring is a single inference, not a multi-step agent). And even where AGT applies, its verifier runs on the operator's own infrastructure — the regulator must trust the operator to run it honestly. No production-grade open, neutral, regulator-runnable verification infrastructure exists today for the broader population of EU high-risk AI systems.

### The solution

Attestly is a small, framework-agnostic library and verification protocol with three composable primitives:

1. **An append-only signed event ledger.** Decisions are signed with the AI system's Ed25519 instance key and appended to a hash-chained ledger (SQLite in the v0 demo, Postgres in production). A database trigger enforces append-only behaviour at the SQL layer.

2. **A publicly-published Signed Tree Head.** Every N events (or on a schedule), the ledger publishes a Merkle root over its canonical payload hashes plus an Ed25519 signature by the operator-organisation's `did:web` key. Only the commitments leave the operator's environment — never the decision payloads themselves.

3. **A standalone verifier.** A regulator (or affected citizen) receives an exported evidence bundle from the operator and runs `attestly verify --bundle` locally. The verifier recanonicalises the decision payload, checks the Merkle inclusion proof against the publicly-published Signed Tree Head, and verifies the Ed25519 signature against the operator's DID document. If the operator altered the decision after publication, the verifier mathematically detects it.

The wire format (Decision Schema v0.1 — a CloudEvents v1.0 profile with Attestly extension attributes) and verifier protocol are versioned and published as open specifications, intended to be implementable by alternative codebases.

### Why this is digital base infrastructure

Attestly is not a SaaS product. It is the *protocol layer* on which AI accountability can be built. The same pattern that the Sovereign Tech Fund supports for `curl` (network base) and OpenSSL (crypto base) applies here: a small, reusable, open-source library that every EU high-risk AI operator can deploy without licensing or vendor lock-in, and that any regulator can run independently without depending on a single commercial stack.

The non-user-facing primitives are deliberate. The library has no UI; the verifier is a 200 kB WASM artefact that loads in a browser; the spec is a 5-page Markdown document with a JSON Schema. This is plumbing, not platform.

### What we have already built

By the submission date, Attestly v0.1 is a working PoC:

- **Rust workspace** (3 crates, ~1500 lines) — `attestly-core` (ledger + Merkle + checkpoint + identity), `attestly-cli` (the `attestly` binary), `attestly-verifier` (no-DB-dependency verifier).
- **20 tests passing** — 18 unit tests covering identity round-trips, canonical hashing stability, append-only trigger enforcement, Merkle proof correctness, checkpoint signature verification; 2 end-to-end integration tests covering the full tamper-detection pipeline and malformed-bundle handling.
- **End-to-end demo** — a 90-second screencast (`examples/render_demo.py` produces `attestly-demo.gif`) showing: initialise → append 50 synthetic credit decisions → publish signed checkpoint → export regulator bundle for decision #24 → verify (PASS) → operator tampers with the database bypassing the append-only trigger → re-export → verify (FAIL, with exact hash mismatch).
- **Clippy + rustfmt clean**, multi-OS CI matrix (Linux + macOS + Windows).
- **MIT-aligned licensing** locked: Apache-2.0 for code, CC-BY-4.0 for specifications.

The screencast is embedded at the top of this proposal and reproducible by any reviewer in ~5 minutes (`cargo build --release && bash examples/demo.sh`).

### What the grant funds

Phase 1 of the public-facing roadmap, taking Attestly from v0.1 (working PoC + demo) to v1.0 (production-grade library with TypeScript SDK, web verifier, security audit, standards-body engagement). See "Deliverables" below.

---

## Open-source status

| Artefact | Licence | Repository |
|---|---|---|
| Rust core (`attestly-core`) | Apache-2.0 | `github.com/attestly/attestly` |
| CLI (`attestly-cli`) | Apache-2.0 | same |
| Verifier (`attestly-verifier`) | Apache-2.0 | same |
| Python SDK (Phase 1) | Apache-2.0 | same |
| TypeScript SDK (Phase 2) | Apache-2.0 | same |
| Specifications (Decision Schema, Checkpoint format, Verifier protocol) | CC-BY-4.0 | `github.com/attestly/spec` |
| Reference dashboard (future commercial layer, post-grant) | AGPL-3.0 | separate repository, never inside grant deliverables |

Governance: BDFL pattern documented in `GOVERNANCE.md` from day 1. By the end of the grant period (Phase 2 W4), at least one additional co-maintainer outside the grant-recipient organisation will be invited (target: an EU-resident contributor with eIDAS / DID experience).

No other public entity has made or is making a grant or investment for the same work, in accordance with Sovereign Tech Fund's eligibility requirements. (A parallel application is in flight to the EIC Accelerator Step 1 for a complementary commercial-layer concept — those funds, if awarded, would build the proprietary hosted-dashboard layer that sits **on top of** the open-source artefacts funded here, not duplicate them. The EIC submission and this submission are scoped for disjoint deliverables.)

---

## Deliverables and milestones

### M1 (end of week 4) — Production-quality core + Python SDK

- Rust workspace passes `cargo audit` + `cargo deny` clean
- Performance baseline published: ≤ 2 ms median append, ≤ 10 ms tree root over 10k leaves
- Python SDK (`attestly` on PyPI) with PyO3 bindings + adapter framework
- Spec v0.1 frozen and published at `spec.attestly.dev`
- First non-Conforme pilot user named publicly

### M2 (end of week 8) — Production transparency log + web verifier

- Migration from MVP file-based STHs to Tessera (`transparency-dev`) Postgres-backed personality
- `logs.attestly.dev` live, serving signed checkpoints and inclusion proofs over HTTPS
- `verify.attestly.dev` — a 200 kB WASM artefact loaded by a vanilla HTML page (no JS framework dependency). Drag-and-drop a `regulator-N.zip` bundle, get a verdict in the browser.
- TypeScript SDK published to npm

### M3 (end of week 12) — Article 12 evidence pack + independent security review

- `attestly export --article-12` produces a regulator-ready bundle covering an arbitrary date range: log entries, all checkpoints covering the range, all inclusion proofs, did:web document, README cross-mapping every Article 12 sub-requirement to a concrete artefact in the bundle.
- Independent security review by Radically Open Security (or equivalent) commissioned at start of M2, report received at end of M3, all blocker findings resolved.
- Compliance mapping documents: `docs/eu-ai-act-article-12-mapping.md`, `docs/soc2-mapping.md`, `docs/gdpr-posture.md`.

### M4 (end of week 16) — v1.0 + standards engagement

- v1.0.0 tagged and published to crates.io + PyPI + npm
- Microsoft AGT bridge adapter shipped (ingests AGT CloudEvents into Attestly format) — proves multi-vendor neutrality
- Decision Schema submitted to CNCF CloudEvents Working Group or W3C Community Group (decision documented in `docs/standards-strategy.md`)
- IETF SCITT WG participation: position document or Internet-Draft mapping Attestly's transparency-log model onto SCITT receipt patterns
- FOSDEM 2027 + IIW Spring 2027 talk submissions filed

Each milestone unblocks 25% of the grant disbursement.

---

## Schedule

16 weeks from grant acceptance (post-scoping call). Each milestone above corresponds to a 4-week segment. The schedule is realistic for a solo lead developer at 60% allocation, with Conforme (the applicant's existing commercial product) providing the remaining 40% of working time and the second pilot integration target. Buffer of ~10% is built into M3 to absorb the security-review feedback loop.

---

## Budget (€100,000)

| Line item | Amount | Notes |
|---|---|---|
| Core engineering — Rust core, Python SDK, verifier CLI, first pilot integration (12 weeks @ €5,000/week) | €60,000 | Phase 0 + Phase 1 core build |
| Integration & release engineering — TypeScript SDK + WASM verifier + multi-platform CI + package publication (crates.io, PyPI, npm) | €15,000 | Phase 2 deliverables |
| Specifications + standards-body engagement — Decision Schema v1.0 spec, Checkpoint format spec, Verifier protocol spec, Article 12 mapping doc, CNCF/W3C submission | €8,000 | M2–M4 |
| Community + governance + maintainer coordination — RFC process setup, contributor onboarding, pilot-user coordination | €4,000 | M1–M4 |
| Production transparency-log infrastructure — Tessera-backed hosted log + static hosting + CDN for verifier UI | €3,000 | M2–M4 hosting |
| Translation of regulator-facing UI (PT + ES + DE + FR) | €3,000 | M3 |
| Independent security review (Radically Open Security or equivalent) | €5,000 | M2–M3, budgeted defensively; Sovereign Tech Fund typically funds review separately on top of grants — if so, this line returns to the programme |
| Travel — FOSDEM 2027 (Brussels) + Article 12 / regulator engagement events | €2,000 | M4 |
| **Total** | **€100,000** | |

Project cost exceeds the €50,000 minimum eligibility threshold. All amounts VAT-exclusive (Portugal-based applicant; VAT treatment per Sovereign Tech Fund's standard scoping process).

---

## Team and expertise

**Lead developer**: Curtis (Portugal-based founder, sole technical lead). 15+ years software engineering, multiple shipped products (Conforme — EU short-term rental compliance SaaS, currently in production with paying customers in Portugal + Spain; Solana / blockchain work; multi-agent AI systems orchestration). Demonstrated track record of solo end-to-end delivery on regulated-industry SaaS. Has internalised the EU AI Act through 6+ months of compliance work on Conforme (the operator's Layer 2 NRUA registration wizard, which itself emits decisions that would fall under Article 12 if classified as high-risk — and which is the natural first pilot for Attestly).

**Co-maintainer**: One additional contributor invited by M2 to satisfy the multi-maintainer neutrality criterion (no single-applicant lock-in on the spec or namespace). Targets identified: an EU-resident contributor with prior `spruceid/ssi`, Sigstore, or `transparency-dev` experience.

**Advisory pilot relationships**: Conforme (commercial), one academic institution (target: INESC TEC / Fraunhofer SIT — outreach in flight), one civil-society organisation (target: AlgorithmWatch / EDRi).

---

## Public benefit and digital sovereignty

Attestly delivers **regulator-grade verification infrastructure for the EU AI accountability ecosystem** without dependence on a single commercial AI vendor's stack. The closest existing capability (Microsoft Agent Governance Toolkit) is MIT-licensed and well-engineered, but:

1. Scopes only to **agentic** AI (a small minority of Annex III high-risk systems);
2. Provides only **in-process** verification (regulator must trust operator's infrastructure);
3. Uses an explicitly **non-stable Decision BOM schema** (per AGT's own documentation), which makes long-term third-party tool ecosystems untenable;
4. Centres the **OpenAI Agents SDK** in its primary integration story.

Each of these is reasonable for a vendor-led project. None are acceptable for the *base infrastructure layer* on which a **regulator–citizen–operator trust triangle** should rest. Attestly is the framework-agnostic open commons that fills the gap.

**The digital sovereignty argument**: without open verification infrastructure, EU AI Act enforcement is downgraded from "the regulator can verify" to "the regulator must trust the operator's verification". That delta is the difference between a binding regulation with admissible evidence and an unenforceable one with operator self-attestation. Attestly restores **evidence admissibility** as a property of the regulatory framework itself, not as a service the operator chooses to provide.

**The compliance-burden argument**: ENISA preparatory analysis estimates Article 12 compliance costs at €0.5–10M per high-risk system, with audit-log integrity cited as the largest unresolved sub-problem. A standardised open primitive reduces this from a per-operator custom build to a drop-in dependency. The cost of compliance for the next 20,000–50,000 EU operators of high-risk AI systems falls measurably as a direct consequence of this work being publicly available.

---

## Sustainability beyond the grant

The library, CLI, verifier, and specifications remain Apache-2.0 + CC-BY-4.0 in perpetuity. Post-grant maintenance is funded through three independent channels:

1. **Conforme commercial revenue** — the applicant's existing SaaS product has paying customers and provides the lead developer's primary income, decoupling Attestly's continued maintenance from grant cycles.
2. **A separate hosted-dashboard service** built *on top of* the open commons artefacts. Open core remains free; the hosted multi-system management dashboard is the commercial sustainability layer. Licensing: AGPL-3.0 for open distribution, proprietary for the hosted SaaS. The dashboard never imports private functionality back into the open libraries.
3. **Follow-on grant submission to NLnet NGI Zero Commons (1 October 2026 cycle)** for Phase 3 scope (federated multi-org logs, EUDI Wallet binding for operator identity, selective-disclosure primitives). By the time of that submission, Sovereign Tech Fund deliverables are shipped and provide track-record evidence.

---

## References, related work, and prior art

- **EU AI Act, Article 12**: https://artificialintelligenceact.eu/article/12/
- **Certificate Transparency (RFC 6962)** — the architectural precedent.
- **Sigstore Rekor** (`github.com/sigstore/rekor`) — the modern transparency-log reference architecture for software supply chains.
- **transparency-dev Tessera** (`github.com/transparency-dev/tessera`) — Trillian's GA successor; Attestly's M2 production log backend.
- **Microsoft Agent Governance Toolkit** (`github.com/microsoft/agent-governance-toolkit`) — the closest market reference; Attestly differentiates on the four axes documented above.
- **"Auditable Agents"** (arxiv:2604.05485) — the academic foundation distinguishing *observability* from *auditability*.
- **`spruceid/ssi`** — Trail-of-Bits-audited DID library, Attestly's identity primitive.
- **CloudEvents v1.0** (CNCF) — Attestly Decision Schema's underlying envelope format.

---

## Demo

A 90-second tamper-detection screencast is available at: `attestly.dev/demo` (post-domain-registration) or attached as `attestly-demo.gif` to this submission.

The complete source code, including all 20 tests and the demo screencast renderer, is available at: `github.com/attestly/attestly` (public from day 1 of submission window).

---

*Submission prepared 2026-05-22. Contact: curts152@gmail.com.*
