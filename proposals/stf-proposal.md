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

Attestly is open digital base infrastructure that closes this gap. It records AI decision events in an append-only signed ledger, publishes Merkle-rooted Signed Tree Heads to a public transparency log, and provides a regulator-runnable verifier that requires no operator cooperation to validate any single decision. The architectural pattern is Certificate Transparency (RFC 9162) applied to AI accountability — **public commitments, private payloads** (GDPR-compatible by construction). The same primitive is positioned to satisfy eIDAS 2 Article 45l Qualified Electronic Ledger requirements (full implementation December 2026), giving Attestly-anchored evidence a legal presumption of authenticity. Apache-2.0 + CC-BY-4.0 in perpetuity. No commercial dependency.

---

## Project description (long form, ~1700 words)

### The problem

The EU AI Act (Regulation EU 2024/1689) categorises a wide range of AI systems as "high-risk" under Annex III: credit scoring, hiring decisions, biometric identification, employment screening, access to essential services, law enforcement, migration management, and judicial decisions. From **2 August 2026**, every operator of such a system must maintain automatic logs over the system's full lifecycle (Article 12), retained for at least 6 months and accessible to market surveillance authorities (Article 26(6)). Operators that fail to comply face penalties up to **€15M or 3% of global turnover** (Article 99).

Article 12 is explicit about *what* must be logged (events sufficient to support post-market monitoring under Article 72) but **silent on integrity**. A determined operator with admin access to their own audit-log database can alter records after the fact. A regulator, presented with an audit log on demand, has no way to verify whether that log is the original record or a rewritten version. The legal framework is in place; the technical assurance is not.

This gap is not theoretical. The same gap exists in every high-stakes domain where a single party both produces and stores evidence about its own conduct — and it was solved decades ago by transparency logs. Certificate Transparency (RFC 9162) eliminated rogue TLS certificate issuance by forcing Certificate Authorities to publish a Merkle-rooted log of every certificate they issue, with public verifiability. Sigstore's Rekor applied the same pattern to software-supply-chain attestations. Microsoft's Agent Governance Toolkit (April 2026) is the first widely-marketed application of tamper-evident audit logs to AI systems — but only to agentic systems within Microsoft's own framework stack, with in-process verification.

Most high-risk AI under Annex III is **not agentic** (credit scoring is a single inference, not a multi-step agent). And even where AGT applies, its verifier runs on the operator's own infrastructure — the regulator must trust the operator to run it honestly. No production-grade open, neutral, regulator-runnable verification infrastructure exists today for the broader population of EU high-risk AI systems.

### The regulatory tailwind — and the C2PA gap

Two converging EU regulatory deadlines shape Attestly's scope:

1. **EU AI Act Article 12** — enforceable 2 August 2026. Mandates the audit log. Silent on integrity.
2. **eIDAS 2 Regulation (EU) 2024/1183, Article 45l** — full implementation December 2026. **Recognises Qualified Electronic Ledgers with legal presumption of authenticity.** For the first time, the EU regulatory framework names cryptographic ledgers as evidentiary infrastructure.

Article 12 mandates the log; Article 45l makes the right kind of log legally evidentiary. Attestly is designed against both — the same Merkle-anchored, signed-checkpoint architecture satisfies both requirements with a single open implementation.

In the content-provenance space, **C2PA (Coalition for Content Provenance and Authenticity)** has been adopted at scale by Adobe, Microsoft, BBC, AP, Reuters, NYT, France Télévisions, and others, and ProofMode (Guardian Project) is C2PA 2.3 conformant as of v3.0.3 RC3 (May 2026). But academic researchers in arXiv 2604.24890 documented a timestamp-substitution attack class on C2PA, alongside an expiry pathology: C2PA-signed media expires, and Arizona Secretary of State's January 2025 pilot images already fail validation a year later. The paper concludes literally: *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."*

Attestly is not a C2PA competitor — for newsroom content provenance, C2PA is the right tool. But the legal-evidence gap C2PA's authors flag is the same gap Article 12 leaves open. Attestly is the append-only transparency log that fixes the timestamp weakness for high-stakes regulatory and legal-evidence use cases, same role Sigstore Rekor plays for software supply chains.

### The solution

Attestly is a small, framework-agnostic library and verification protocol with three composable primitives:

1. **An append-only signed event ledger.** Decisions are signed with the AI system's Ed25519 instance key and appended to a hash-chained ledger (SQLite in the v0 demo, Postgres in production). A database trigger enforces append-only behaviour at the SQL layer; cryptographic chaining enforces it at the storage layer.

2. **A publicly-published Signed Tree Head.** Every N events (or on a schedule), the ledger publishes a Merkle root over its canonical payload hashes plus an Ed25519 signature by the operator-organisation's `did:web` key. Only the commitments leave the operator's environment — never the decision payloads themselves.

3. **A standalone verifier.** A regulator (or affected citizen) receives an exported evidence bundle from the operator and runs `attestly verify --bundle` locally. The verifier recanonicalises the decision payload, checks the Merkle inclusion proof against the publicly-published Signed Tree Head, and verifies the Ed25519 signature against the operator's DID document. If the operator altered the decision after publication, the verifier mathematically detects it.

The wire format (Decision Schema v0.1 — a CloudEvents v1.0 profile with Attestly extension attributes) and verifier protocol are versioned and published as open specifications, intended to be implementable by alternative codebases.

### Why this is digital base infrastructure

Attestly is not a SaaS product. It is the *protocol layer* on which AI accountability can be built. The same pattern that the Sovereign Tech Fund supports for `curl` (network base) and OpenSSL (crypto base) applies here: a small, reusable, open-source library that every EU high-risk AI operator can deploy without licensing or vendor lock-in, and that any regulator can run independently without depending on a single commercial stack.

The non-user-facing primitives are deliberate. The library has no UI; the verifier is a 200 kB WASM artefact that loads in a browser; the spec is a 5-page Markdown document with a JSON Schema. This is plumbing, not platform.

### Breadth of applicability

The primary surface is EU AI Act Article 12 — that is what the August 2026 deadline forces and what this grant addresses. But the underlying primitive is general. The same append-only ledger, the same Signed Tree Heads, the same regulator-runnable verifier serve any domain where a single party both produces and stores evidence about its own conduct. Two adjacent applications are already in flight under parallel grant proposals, and inform the v1.0 design without changing the STF deliverable scope:

- **Civil-society and journalism field evidence** — captured material from journalists in restricted-press environments and human-rights documenters working in authoritarian contexts. Integration adapter under development for **Tella** (Horizontal); partnership conversation initiated with the **International Bar Association's eyeWitness to Atrocities** programme. Funded separately by the Open Technology Fund Internet Freedom Fund (concept note submitted) — STF deliverables remain focused on the AI Act surface.
- **Federated multi-organisation logs and EUDI Wallet operator identity** — Phase 3 scope addressed by a parallel NLnet NGI Zero Commons application (October 2026 cycle).

These adjacent applications validate that the primitive Attestly ships is genuinely framework-agnostic and not narrowly AI-Act-specific. They do not appear in this STF deliverable list.

### What we have already built

By the submission date, Attestly v0.1 is a working PoC:

- **Rust workspace** (3 crates + adapter scaffold, ~1,500 lines) — `attestly-core` (ledger + Merkle + checkpoint + identity), `attestly-cli` (the `attestly` binary), `attestly-verifier` (no-DB-dependency verifier), `attestly-tella` (scaffold for the OTF-funded civil-society integration; out of STF scope).
- **40+ tests passing** — 20 in `attestly-core` covering identity round-trips, canonical hashing stability, append-only trigger enforcement, Merkle proof correctness, checkpoint signature verification, full end-to-end tamper-detection pipeline, malformed-bundle handling; 20 in `attestly-tella` covering upload canonicalisation, evidence-event derivation, receipt schema, and privacy invariants (raw payloads never appear in events).
- **End-to-end demo** — a 27-second screencast (`examples/render_demo.py` produces `attestly-demo.gif`) showing: initialise → append 50 synthetic credit decisions → publish signed checkpoint → export regulator bundle for decision #24 → verify (PASS) → operator tampers with the database bypassing the append-only trigger → re-export → verify (FAIL, with exact hash mismatch).
- **Clippy + rustfmt clean**, multi-OS CI matrix (Linux + macOS + Windows).
- **Public project site** at https://attestly.org with the demo embedded and architecture documented.
- **Licensing locked**: Apache-2.0 for code, CC-BY-4.0 for specifications.

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

Governance: BDFL pattern documented in `GOVERNANCE.md` from day 1. By the end of the grant period (Phase 2 W4), at least one additional co-maintainer outside the grant-recipient organisation will be invited (target: an EU-resident contributor with eIDAS / DID / `transparency-dev` experience).

---

## Parallel applications and non-overlap statement

Parallel grant applications are in flight or in preparation, each scoped to disjoint deliverables that do not duplicate this STF Phase 1 work. The boundaries are documented in `proposals/README.md`:

- **Open Technology Fund — Internet Freedom Fund** (concept note submitted): civil-society and journalism evidence-integrity scope — specifically the Tella integration adapter, browser WASM verifier with multilingual UI, IETF SCITT spec submission, and outreach to the International Bar Association's eyeWitness team. None of these deliverables appear in this STF list.
- **NLnet NGI Zero Commons, October 2026 cycle** (in preparation): Phase 3 scope — federated multi-organisation transparency logs and EUDI Wallet operator-identity binding. Out of STF Phase 1 scope.
- **Internet Society Foundation Research Grant 2026** (in preparation): empirical study scope — peer-reviewed analysis of public-anchor primitives across three adversarial threat models. Research outputs, not engineering deliverables.
- **EIC Accelerator Step 1** (in preparation): unlocks the **ANI Voucher Deep Tech (€60k + €10k consultancy)** mechanically on a GO, funding a proprietary hosted-dashboard layer that sits *on top of* the open library funded here, never duplicating it.

No deliverable is funded twice. The applicant will withdraw any application or rescope any deliverable that Sovereign Tech Fund identifies as overlapping with another funded grant.

---

## Deliverables and milestones

### M1 (end of week 4) — Production-quality core + Python SDK

- Rust workspace passes `cargo audit` + `cargo deny` clean
- Performance baseline published: ≤ 2 ms median append, ≤ 10 ms tree root over 10k leaves
- Python SDK (`attestly` on PyPI) with PyO3 bindings + adapter framework
- Spec v0.1 frozen and published at `spec.attestly.org`
- First non-Conforme pilot user named publicly

### M2 (end of week 8) — Production transparency log + web verifier

- Migration from MVP file-based STHs to Tessera (`transparency-dev`) Postgres-backed personality
- `logs.attestly.org` live, serving signed checkpoints and inclusion proofs over HTTPS
- `verify.attestly.org` — a 200 kB WASM artefact loaded by a vanilla HTML page (no JS framework dependency). Drag-and-drop a `regulator-N.zip` bundle, get a verdict in the browser.
- TypeScript SDK published to npm

### M3 (end of week 12) — Article 12 evidence pack + independent security review

- `attestly export --article-12` produces a regulator-ready bundle covering an arbitrary date range: log entries, all checkpoints covering the range, all inclusion proofs, did:web document, README cross-mapping every Article 12 sub-requirement to a concrete artefact in the bundle.
- Independent security review by Radically Open Security (or equivalent) commissioned at start of M2, report received at end of M3, all blocker findings resolved.
- Compliance mapping documents: `docs/eu-ai-act-article-12-mapping.md`, `docs/eidas-2-article-45l-mapping.md`, `docs/soc2-mapping.md`, `docs/gdpr-posture.md`.

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
| Specifications + standards-body engagement — Decision Schema v1.0 spec, Checkpoint format spec, Verifier protocol spec, Article 12 + eIDAS 2 Art 45l mapping docs, CNCF/W3C submission | €8,000 | M2–M4 |
| Community + governance + maintainer coordination — RFC process setup, contributor onboarding, pilot-user coordination | €4,000 | M1–M4 |
| Production transparency-log infrastructure — Tessera-backed hosted log + static hosting + CDN for verifier UI | €3,000 | M2–M4 hosting |
| Translation of regulator-facing UI (PT + ES + DE + FR) | €3,000 | M3 |
| Independent security review (Radically Open Security or equivalent) | €5,000 | M2–M3, budgeted defensively; Sovereign Tech Fund typically funds review separately on top of grants — if so, this line returns to the programme |
| Travel — FOSDEM 2027 (Brussels) + Article 12 / regulator engagement events | €2,000 | M4 |
| **Total** | **€100,000** | |

Project cost exceeds the €50,000 minimum eligibility threshold. All amounts VAT-exclusive (Portugal-based applicant; VAT treatment per Sovereign Tech Fund's standard scoping process).

---

## Team and expertise

**Lead developer**: Curtis (Portugal-based founder, sole technical lead). 15+ years software engineering, multiple shipped products (Conforme — EU short-term rental compliance SaaS, currently in production with paying customers in Portugal; Solana / blockchain work; multi-agent AI systems orchestration). Demonstrated track record of solo end-to-end delivery on regulated-industry SaaS. Has internalised the EU AI Act through 6+ months of compliance work on Conforme (the operator's Layer 2 NRUA registration wizard, which itself emits decisions that would fall under Article 12 if classified as high-risk — and which is the natural first pilot for Attestly).

**Co-maintainer**: One additional contributor invited by M2 to satisfy the multi-maintainer neutrality criterion (no single-applicant lock-in on the spec or namespace). Targets identified: an EU-resident contributor with prior `spruceid/ssi`, Sigstore, or `transparency-dev` experience.

**Advisory pilot relationships** (in progress as of submission):
- **Conforme** (commercial pilot — applicant's existing SaaS product)
- **Horizontal / Tella maintainers** (civil-society pilot — adapter under development for OTF Phase 2 grant)
- **International Bar Association / eyeWitness to Atrocities** (partnership conversation — court-admissible field evidence)
- One Portuguese academic institution (target: INESC TEC CRACS — outreach sent 2026-05-26 for IAPMEI Vale Inovação cryptographic-review partnership)

---

## Public benefit and digital sovereignty

Attestly delivers **regulator-grade verification infrastructure for the EU AI accountability ecosystem** without dependence on a single commercial AI vendor's stack. The closest existing capability (Microsoft Agent Governance Toolkit) is MIT-licensed and well-engineered, but:

1. Scopes only to **agentic** AI (a small minority of Annex III high-risk systems);
2. Provides only **in-process** verification (regulator must trust operator's infrastructure);
3. Uses an explicitly **non-stable Decision BOM schema** (per AGT's own documentation), which makes long-term third-party tool ecosystems untenable;
4. Centres the **OpenAI Agents SDK** in its primary integration story.

Each of these is reasonable for a vendor-led project. None are acceptable for the *base infrastructure layer* on which a **regulator–citizen–operator trust triangle** should rest. Attestly is the framework-agnostic open commons that fills the gap.

**The digital sovereignty argument**: without open verification infrastructure, EU AI Act enforcement is downgraded from "the regulator can verify" to "the regulator must trust the operator's verification". That delta is the difference between a binding regulation with admissible evidence and an unenforceable one with operator self-attestation. Attestly restores **evidence admissibility** as a property of the regulatory framework itself, not as a service the operator chooses to provide. The eIDAS 2 Article 45l recognition of Qualified Electronic Ledgers gives this work a second EU regulatory anchor independent of AI Act enforcement timelines.

**The compliance-burden argument**: ENISA preparatory analysis estimates Article 12 compliance costs at €0.5–10M per high-risk system, with audit-log integrity cited as the largest unresolved sub-problem. A standardised open primitive reduces this from a per-operator custom build to a drop-in dependency. The cost of compliance for the next 20,000–50,000 EU operators of high-risk AI systems falls measurably as a direct consequence of this work being publicly available.

---

## Sustainability beyond the grant

The library, CLI, verifier, and specifications remain Apache-2.0 + CC-BY-4.0 in perpetuity. Post-grant maintenance is funded through four independent channels:

1. **Conforme commercial revenue** — the applicant's existing SaaS product has paying customers and provides the lead developer's primary income, decoupling Attestly's continued maintenance from grant cycles.
2. **A separate hosted-dashboard service** built *on top of* the open commons artefacts. Open core remains free; the hosted multi-system management dashboard is the commercial sustainability layer. Licensing: AGPL-3.0 for open distribution, proprietary for the hosted SaaS. The dashboard never imports private functionality back into the open libraries.
3. **Parallel grants for adjacent scope**: OTF Internet Freedom Fund concept note submitted (civil-society Phase 2); NLnet NGI Zero Commons October 2026 cycle (Phase 3 federated logs + EUDI Wallet binding); ISF Research Grant 2026 (empirical study). By the time STF deliverables ship, these provide track-record evidence and parallel funding streams that decouple Attestly's continuity from any single grant outcome.
4. **EIC Accelerator Step 1 + ANI Voucher Deep Tech** (€70k mechanical on GO): funds the proprietary commercial layer that sits on top of the STF-funded open library, not the library itself.

---

## References, related work, and prior art

- **EU AI Act, Article 12**: https://artificialintelligenceact.eu/article/12/
- **eIDAS 2 Regulation (EU) 2024/1183, Article 45l**: https://eur-lex.europa.eu/eli/reg/2024/1183 (Qualified Electronic Ledgers)
- **Certificate Transparency (RFC 9162)** — the architectural precedent: https://datatracker.ietf.org/doc/rfc9162/
- **Sigstore Rekor** (`github.com/sigstore/rekor`) — the modern transparency-log reference architecture for software supply chains.
- **transparency-dev Tessera** (`github.com/transparency-dev/tessera`) — Trillian's GA successor; Attestly's M2 production log backend.
- **Microsoft Agent Governance Toolkit** (`github.com/microsoft/agent-governance-toolkit`) — the closest market reference; Attestly differentiates on the four axes documented above.
- **C2PA timestamp-substitution and expiry analysis**: arXiv [2604.24890](https://arxiv.org/abs/2604.24890) — documents the legal-evidence gap Attestly's transparency log fixes.
- **"Auditable Agents"** (arxiv:2604.05485) — the academic foundation distinguishing *observability* from *auditability*.
- **`spruceid/ssi`** — Trail-of-Bits-audited DID library, Attestly's identity primitive.
- **CloudEvents v1.0** (CNCF) — Attestly Decision Schema's underlying envelope format.
- **Horizontal / Tella** (`github.com/Horizontal-org/Tella-Android`) — civil-society capture tool; Attestly Phase 2 integration target (OTF-funded).
- **International Bar Association / eyeWitness to Atrocities** — institutional partner for evidence-integrity adoption in human-rights casework.

---

## Demo

A 27-second tamper-detection screencast is available at https://attestly.org (embedded on the landing page) or attached as `attestly-demo.gif` to this submission.

The complete source code, including all 40+ tests across the workspace and the demo screencast renderer, is available at: `github.com/attestly/attestly` (public).

---

*Submission prepared 2026-05-26. Contact: hello@attestly.org · curts152@gmail.com.*
