# STF Application Form — Answers v2 (form-structure-accurate)

> Companion to `stf-proposal.md`. Sized for the **actual** fields in the
> apply.sovereigntechfund.de form (extracted from view-source 2026-05-26).
> Two tabs, 29 user-input fields. Per-field word limits aren't exposed in
> the form-config JSON — I've sized each textarea at a sensible default
> (~150-250 words); message me the exact word limit shown for any field
> that needs tighter compression.

The form has two tabs:

- **Tab 1 — Entry-form prescreen** (`kEGEQRWB`)
- **Tab 2 — Sovereign Tech Fund Proposal** (`krzQowYd`)

Fields are listed below in the order they appear in the form structure.

---

# TAB 1 — Entry-form prescreen

## Link to project website (url)

```
https://attestly.org
```

## Link to project repository (url)

```
https://github.com/attestly/attestly
```

## Project title (text, ≤ 1 line)

```
Attestly
```

## project-needs (textarea)

Attestly is at v0.1 — a working PoC with 40+ tests passing, an end-to-end tamper-detection demo, and a public landing page. The library compiles clean across Linux/macOS/Windows. What's needed to reach v1.0 is concentrated, full-time engineering for ~16 weeks across four areas: (a) production-grade core (performance baseline, Python SDK, frozen spec); (b) production transparency log (Tessera migration, browser WASM verifier, TypeScript SDK); (c) regulator-ready Article 12 evidence-pack export + independent security review; (d) standards-body engagement (IETF SCITT, CNCF CloudEvents or W3C Community Group). The applicant maintains a commercial product (Conforme) that provides primary income but only ~40% of working time; STF funding lets the lead developer commit ~60% to Attestly full-bore for the v1.0 sprint and pays for the security review + translation + standards-body coordination that no commercial channel funds.

**~165 words.**

## public-interest (textarea)

EU AI Act Article 12 (enforceable 2 August 2026) mandates audit logging by every high-risk AI system but provides no integrity assurance. A determined operator can edit their own audit trail and no off-the-shelf technology lets a regulator detect the edit. This is a public-interest gap because: (1) regulatory effectiveness depends on it — without it, EU AI accountability is downgraded from "regulator can verify" to "regulator must trust the operator"; (2) it cannot be filled by commercial software, because trust requires independence from the operator's incentives; (3) the same primitive serves civil society (journalist + human-rights field evidence) and the EU's broader eIDAS 2 Article 45l Qualified Electronic Ledger framework (December 2026), so the public-interest value compounds across multiple regulated trust domains.

Attestly's contribution is conventional cryptography (Merkle trees, Ed25519, SHA-256) composed and packaged for a specific regulatory deadline. The composition is the work; the primitives are well-understood. Apache-2.0 in perpetuity removes commercial-dependency risk for every adopter.

**~175 words.**

## services (checkboxlist)

The form doesn't reveal which service options are listed. Likely choices include things like: *library development · security review · specifications · translation · standards engagement · infrastructure hosting · maintenance / sustainability funding*. Recommended ticks:

- [x] **Library development / engineering** (the M1-M4 core build)
- [x] **Independent security review** (M2-M3 deliverable — Radically Open Security or equivalent)
- [x] **Specifications work** (Decision Schema, Checkpoint Format, Verifier Protocol — CC-BY-4.0)
- [x] **Standards engagement** (IETF SCITT, CNCF CloudEvents or W3C Community Group)
- [x] **Translation** (regulator-facing UI in PT + ES + DE + FR)

If "Infrastructure hosting" / "Maintenance / sustainability" is an option, tick those as well (the Tessera production log at `logs.attestly.org` is M2). Screenshot the actual checkbox list and I'll narrow.

## state-of-development (textarea)

Attestly v0.1 is shipped and runnable today:

- **Public repository** at github.com/attestly/attestly (Apache-2.0). Three Rust crates + adapter scaffold, ~1,500 LoC, clippy + rustfmt clean.
- **40+ tests passing** — 20 in `attestly-core` (identity round-trips, canonical-hash stability, append-only enforcement, Merkle proof correctness, checkpoint signature verification, end-to-end tamper-detection pipeline, malformed-bundle handling); 20 in `attestly-tella` (privacy invariants for the parallel civil-society integration scaffold, OTF-funded).
- **End-to-end demo**: a 27-second screencast (`examples/render_demo.py` produces `attestly-demo.gif`) showing initialise → 50 synthetic credit decisions → publish signed checkpoint → export regulator bundle → verify (PASS) → operator tampers → re-verify (FAIL, with exact hash mismatch).
- **Multi-OS CI matrix** (Linux, macOS, Windows).
- **Public website + demo** at https://attestly.org.
- **Reproducible from scratch**: `cargo build --release && bash examples/demo.sh` in ~5 minutes.

TRL equivalent: 4-5 (technology validated in lab; pilot integration with Conforme planned for M1).

**~165 words.**

## userbases (textarea)

Three concentric user groups, in priority order:

**Primary (Phase 1 scope, STF funded)**: operators of EU high-risk AI systems under AI Act Annex III. ENISA preparatory analysis estimates 20,000-50,000 such operators across the EU by enforcement date. Categories: credit-scoring, hiring, biometric identification, employment, essential-services access, law enforcement, migration, judicial. First named pilot is **Conforme** (the applicant's existing commercial PT-based SaaS, whose NRUA registration wizard emits Article-12-shaped decisions).

**Secondary (parallel grants address this)**: regulators (national market-surveillance authorities, the EU AI Office), and affected citizens or courts that need to verify a single decision independently. The browser WASM verifier (M2) targets this group.

**Tertiary (post-grant, broader sectoral applications)**: civil-society and journalism field evidence (Tella integration adapter under OTF Phase 2 funding); cross-sector regulated trust (DORA, CSRD, supply-chain due-diligence regulations) as adjacent applications.

The primary group alone — EU high-risk AI operators — is sufficient to justify the STF Phase 1 scope.

**~175 words.**

---

# TAB 2 — Sovereign Tech Fund Proposal

## Project title (text)

```
Attestly
```

## Name (text)

```
Curtis Smith
```

*(or whichever is on your STF account; some STF forms expect full legal name)*

## Profile (url)

```
https://github.com/Niteowlpt
```

*(or your personal/professional profile of choice — LinkedIn if preferred)*

## Residence (General Fund) — country

```
Portugal
```

## Role (checkboxlist)

Likely options include things like *lead developer / maintainer / contributor / commercial operator / individual / organisation*. Recommended ticks:

- [x] **Lead developer / maintainer**
- [x] **Individual** (currently — Empresário em Nome Individual / sole proprietor)

If there's an "applicant on behalf of an organisation" option, leave that unchecked unless Curtis is applying via a registered Lda.

## Link to project repository (url) — same as Tab 1

```
https://github.com/attestly/attestly
```

## Link to project website (url) — same as Tab 1

```
https://attestly.org
```

## Description (textarea — short summary, ~100 words target)

Attestly is an open-source library that gives any high-risk AI system a tamper-evident audit log. It records each decision in an append-only, cryptographically-signed ledger; publishes only Merkle-rooted commitments to a public transparency log; and exports evidence bundles a regulator can verify locally with a standalone tool. The architecture is Certificate Transparency for AI accountability — public commitments, private payloads, GDPR-compatible by construction. Apache-2.0 in perpetuity, framework-agnostic, no commercial dependency. Designed to satisfy EU AI Act Article 12 (enforceable August 2026) and eIDAS 2 Article 45l Qualified Electronic Ledger requirements (December 2026).

**~95 words.**

## Describe your project more in-depth (textarea — ~300 words)

From 2 August 2026, EU AI Act Article 12 makes automatic decision-logging mandatory for every high-risk AI system — credit scoring, hiring, biometric identification, employment, law enforcement, migration, judicial decisions. Non-compliance reaches €15M or 3% of global turnover. The Act tells operators what to log but not how to make those logs trustworthy.

Today an operator can edit their own audit trail before a regulator inspects it, and no off-the-shelf technology lets the regulator detect the edit. This is the integrity gap the law leaves open. The closest existing tool — Microsoft's Agent Governance Toolkit (April 2026) — scopes only to agentic AI within Microsoft's framework stack, and provides only in-process verification (the regulator must trust the operator's infrastructure). Content-provenance standard C2PA has a documented timestamp-substitution flaw (arXiv 2604.24890) that explicitly excludes it from legal-evidence use cases.

Attestly closes the gap with three composable primitives: an append-only signed event ledger; publicly-published Merkle-rooted Signed Tree Heads (decisions stay private; only short commitments touch public infrastructure); and a regulator-runnable standalone verifier requiring no operator cooperation. The architecture is Certificate Transparency for AI accountability — a pattern with thirteen years of production use in the TLS ecosystem.

Critical now because two EU regulatory deadlines converge: AI Act Article 12 enforceable August 2026; eIDAS 2 Regulation 2024/1183 Article 45l recognises Qualified Electronic Ledgers with legal presumption of authenticity from December 2026 — the first time EU regulatory framework names cryptographic ledgers as evidentiary infrastructure. Attestly is the open reference implementation that bridges both. ENISA preparatory analysis estimates Article 12 compliance costs at €0.5–10M per high-risk system; a standardised open primitive reduces this from per-operator custom build to drop-in dependency.

**~295 words.**

## Please provide a brief overview (textarea — ~200 words target)

Attestly is a small open-source Rust library (~1,500 LoC across three crates) that solves the audit-log integrity gap left open by EU AI Act Article 12. Decisions are canonically hashed, signed with the AI system's Ed25519 key, and appended to a hash-chained ledger. Periodically the ledger publishes a Merkle root + Ed25519 signature to a public transparency log — only short cryptographic commitments leave the operator's environment; decision payloads stay private. A standalone verifier (CLI + browser WASM) takes any exported evidence bundle and detects tampering mathematically, with no operator cooperation needed and no vendor dependency.

The architectural pattern is Certificate Transparency (RFC 9162), already proven across 13 years of the TLS ecosystem, applied to AI accountability. The wire format is a CloudEvents v1.0 profile with Attestly extension attributes; specifications are CC-BY-4.0 and intended to be implementable by alternative codebases. Apache-2.0 in perpetuity for code; the applicant maintains a commercial product (Conforme) that decouples Attestly's continuity from grant cycles. Phase 1 — the work this grant funds — takes the project from v0.1 PoC to v1.0 production-ready over 16 weeks across four 4-week milestones (M1-M4).

**~190 words.**

## Activities (textarea — what the grant funds, ~250-300 words)

Phase 1 — 16 weeks across four milestones, each unblocking 25% of grant disbursement:

**M1 (week 4) — Production-quality core + Python SDK.** Rust workspace passes `cargo audit` + `cargo deny` clean. Performance baseline published: ≤ 2 ms median append, ≤ 10 ms tree root over 10k leaves. Python SDK (`attestly` on PyPI) with PyO3 bindings + adapter framework. Spec v0.1 frozen and published at spec.attestly.org. First non-Conforme pilot user named publicly.

**M2 (week 8) — Production transparency log + browser WASM verifier.** Migration from MVP file-based STHs to Tessera (`transparency-dev`) Postgres-backed personality. `logs.attestly.org` live, serving signed checkpoints and inclusion proofs over HTTPS. `verify.attestly.org` — a 200 kB WASM artefact loaded by a vanilla HTML page (no JS framework dependency); drag-and-drop a regulator bundle, get a verdict in the browser. TypeScript SDK published to npm.

**M3 (week 12) — Article 12 evidence pack + independent security review.** `attestly export --article-12` produces a regulator-ready bundle covering an arbitrary date range, with a README cross-mapping every Article 12 sub-requirement to a concrete artefact. Independent security review by Radically Open Security (or equivalent) commissioned at start of M2, report received end of M3, blocker findings resolved. Compliance mapping documents: Article 12, eIDAS 2 Art 45l, SOC 2, GDPR.

**M4 (week 16) — v1.0 + standards engagement.** v1.0.0 tagged and published to crates.io + PyPI + npm. Microsoft AGT bridge adapter (proves multi-vendor neutrality). Decision Schema submitted to CNCF CloudEvents WG or W3C Community Group. IETF SCITT WG position document or Internet-Draft. FOSDEM 2027 + IIW Spring 2027 talk submissions.

**~285 words.**

## allignment (textarea — alignment with STF mission, ~150 words)

Sovereign Tech Fund supports open digital base technology — small, reusable, framework-agnostic libraries that ecosystems depend on but that no commercial actor has incentive to build openly. STF's existing portfolio (`curl`, OpenSSL, KDE, Postgres infrastructure, Sequoia-PQ, Sigstore) is the same shape of project Attestly is: plumbing, not platform; protocol layer, not product. AI accountability infrastructure is the same problem applied on a one-year regulatory timescale.

STF specifically (vs other open-source funders) because: the funding band matches Phase 1 scope without forcing artificial scope-up; STF accepts rolling submissions and evaluates on technical merit, not quarterly competitive cycles; STF's bug-bounty and security-review programmes complement the M2-M3 independent-review budget; STF's track record of funding base-layer EU digital infrastructure positions Attestly inside a portfolio narrative EU regulators are increasingly aware of.

**~150 words.**

## challenges in maintenance (textarea — what makes ongoing maintenance hard, ~150 words)

Three structural maintenance challenges:

1. **Specification stability across the AI Act enforcement curve.** The European Commission and national competent authorities will publish implementing guidance for Article 12 over 2026-2028. The Decision Schema may need to track evolving regulator expectations without breaking earlier-version evidence bundles. Mitigation: versioned spec from day 1 (v0.1, v1.0, v2.0…), with a regulator-facing mapping document that updates incrementally.

2. **Single-lead-developer risk.** Attestly is currently maintained by one person; long-term sustainability requires multi-maintainer governance. Mitigation: co-maintainer onboarded by M2 (governance criterion); the BDFL pattern is documented in `GOVERNANCE.md` from day 1.

3. **Funding cliff after Phase 1.** Grant work ends at v1.0; ongoing maintenance needs a sustainability story. Mitigation: applicant's existing commercial product (Conforme) provides primary income, decoupling Attestly's maintenance from grant cycles; parallel grants (OTF, NLnet NGI0 Oct, ISF) extend funded engineering through 2027.

**~165 words.**

## concrete scenario (textarea — specific use-case story, ~200 words)

A Portuguese consumer loan applicant is rejected by an AI-driven credit-scoring system covered by AI Act Annex III, and disputes the decision. The applicant requests a copy of the audit log under GDPR Article 15 and the AI Act's record-keeping obligation. The operator complies and produces a log entry showing the rejection.

Without Attestly, the applicant has no way to verify whether the log entry is the original record. If the operator tampered with the entry after the rejection — to remove a discriminatory input feature, for example — there is no technical mechanism for the applicant, their lawyer, or the Portuguese national market-surveillance authority (ANCP) to detect it.

With Attestly, the operator includes in its disclosure an Attestly evidence bundle covering the rejection decision: the canonicalised event, the Ed25519 signature, the Merkle inclusion proof, and a reference to the Signed Tree Head published at `logs.attestly.org` covering the relevant date range. The applicant drops the bundle into `verify.attestly.org` in their browser. The verifier mathematically confirms the disclosed record matches what was publicly committed at the time of the decision, or detects the discrepancy.

**~205 words.**

## Dependencies (textarea — project's dependencies on other OSS, ~150 words)

Attestly composes well-understood cryptographic primitives from established Rust crates:

- **ed25519-dalek** (Rust ed25519 implementation, Trail-of-Bits-audited) — operator and instance signatures.
- **sha2** (SHA-256, NIST-standardised) — canonical hashing.
- **rs_merkle** (Merkle tree library) — Signed Tree Head construction.
- **rusqlite** (SQLite bindings) — append-only event ledger for the v0.1 PoC and embedded deployments.
- **serde + serde_json** (canonical serialisation) — Decision Schema events.
- **transparency-dev/tessera** (Trillian successor, M2 production-log backend) — Go implementation embedded as a backend service.
- **spruceid/ssi** (Trail-of-Bits-audited DID library) — operator identity (`did:web`).

Tessera and Sigstore Rekor are conceptual prior art; Attestly does not re-implement either, instead consuming them where applicable. CloudEvents v1.0 (CNCF) provides the underlying envelope format. No proprietary dependencies; no GPL-encumbered dependencies (all Apache-2.0, MIT, BSD, or equivalent permissive licences).

**~140 words.**

## Recipients (textarea — who receives the funds within the project, ~100 words)

The grant recipient is **Curtis Smith** (Portugal-based, sole proprietor — Empresário em Nome Individual). All Phase 1 engineering is performed by the applicant directly; no sub-contracting to external agencies for core code.

Three external payments are budgeted as line items:

- **Independent security review** (€5,000) — paid to Radically Open Security or equivalent EU-based firm.
- **Translation services** (€3,000) — paid to a professional translation agency for the PT/ES/DE/FR regulator-facing UI strings.
- **Production infrastructure** (€3,000) — paid to Cloudflare, Fastly, or equivalent for hosted transparency-log + CDN.

If a co-maintainer is onboarded with paid honorarium (alternative to ecosystem-volunteer model), the line is drawn from the community/governance line.

**~115 words.**

## Target groups (textarea — who benefits, ~150-200 words)

**Primary** — operators of EU high-risk AI systems under AI Act Annex III (~20,000-50,000 organisations across the EU by ENISA estimate). Drop-in tamper-evident audit logging replaces per-operator custom integrity engineering, reducing Article 12 compliance cost from €0.5-10M per system to a library dependency.

**Primary** — EU and national market-surveillance authorities, the EU AI Office, national data-protection authorities. Regulator-runnable verification means enforcement effectiveness no longer depends on operator self-attestation.

**Primary** — affected citizens, their lawyers, and courts. A standalone browser verifier lets any individual challenge an AI decision's authenticity with a single drag-and-drop, without depending on the operator's cooperation or any vendor's stack.

**Secondary** — civil-society organisations and journalists capturing court-admissible evidence (Tella, eyeWitness ecosystem). Adjacent application funded by parallel grants, validating that the primitive is framework-agnostic.

**Secondary** — standards bodies and adjacent OSS projects. The Decision Schema submission to CNCF CloudEvents WG or W3C Community Group; the Verifier Protocol submission to IETF SCITT WG. Both increase ecosystem-wide reusability.

**~190 words.**

## Past funding (textarea — prior grants received, ~100 words)

The applicant has not previously received any grant from Sovereign Tech Fund.

Conforme (the applicant's existing commercial product) is bootstrapped from operating revenue and has not received public grants to date; Conforme's grant pipeline (SICE Internacionalização, SICE Empreendedorismo Qualificado) is separate from Attestly and addresses commercial-scale-up needs, not Attestly's open-source library work.

Parallel applications in flight or in preparation for Attestly: Open Technology Fund Internet Freedom Fund (concept note submitted); NLnet NGI Zero Commons October 2026 cycle (in preparation); Internet Society Foundation Research Grant 2026 (in preparation); EIC Accelerator Step 1 + ANI Voucher Deep Tech (in preparation). Non-overlap statement covers each.

**~115 words.**

## What are possible alternatives? (textarea — ~150-200 words)

Three categories of existing alternatives, each insufficient for the EU AI Act Article 12 regulatory-evidence gap:

**1. Closed proprietary AI audit platforms** (Datadog, AgentOps, Helicone, Langfuse). Operator-side telemetry; the regulator sees what the operator chooses to show. None publish cryptographic commitments to public infrastructure. Not regulator-runnable, not vendor-neutral, not open-source.

**2. Microsoft Agent Governance Toolkit** (April 2026, MIT). The closest single market reference. Scopes only to agentic AI within Microsoft's framework stack (small minority of Annex III high-risk); provides only in-process verification (regulator must trust operator's infrastructure); uses explicitly non-stable Decision BOM schema; centres OpenAI Agents SDK. Reasonable for vendor-led; insufficient as base infrastructure.

**3. Content-provenance standards** (C2PA / Content Credentials). Adopted at scale by Adobe, Microsoft, BBC, AP, Reuters, NYT, France Télévisions. But: documented timestamp-substitution flaw (arXiv 2604.24890) and signed-media expiry pathology (Arizona January 2025 pilot images already fail validation). Authors literally conclude *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."* C2PA is right for newsroom provenance; insufficient for the regulatory-evidence integrity gap.

**~195 words.**

## hear about (checkboxlist — how did you hear about STF)

The form doesn't reveal options. Common STF choices include: *direct referral / OSS community / conference talk / press article / search / social media / Sovereign Tech Fund Summit*. Tick whichever is honest (Curtis to fill in).

## cost (numeric — total grant ask in EUR)

```
100000
```

## Estimate (numeric — likely a separate field for STF's own estimate or a sub-total)

If this field is the **total project cost including co-financing**, the answer is the same as `cost`: `100000`. STF Phase 1 doesn't require co-financing the way ESA does.

If it's a **per-task estimate** or **STF's internal estimate field**, leave blank and screenshot the field's help text — I'll re-interpret.

## Timeframe (numeric — likely weeks or months)

```
16
```

(weeks — corresponds to 4 milestones × 4 weeks each)

If the form expects months: `4`. Screenshot the unit label and I'll confirm.

## checkbox financing

Likely confirms: "I have read and understood the financing model" or "I will not double-finance this work with other public grants". Tick **yes** — the parallel applications cover disjoint scope and the non-overlap statement is in `Past funding`.

## checkbox legal

Likely confirms: "I am legally able to enter into this agreement" or "I have the legal capacity to receive grant funds". Tick **yes** (Curtis is a Portuguese ENI / sole proprietor with legal capacity).

## checkbox license

Likely confirms: "I will publish the funded work under an open-source licence acceptable to STF". Tick **yes** — Apache-2.0 + CC-BY-4.0 are both OSI/OSI-equivalent approved.

## checkbox receiving notifications

Tick **yes** — Curtis wants to receive STF programme updates and notifications about this application.

---

# Submission readiness checklist

Before clicking submit, confirm:

- [ ] All textareas pasted and within their actual word limits (resize on demand if any limit is tighter than the ~150-250 word defaults used here)
- [ ] All URLs verified to load (attestly.org, github.com/attestly/attestly)
- [ ] Numeric fields filled (cost: 100000; Timeframe: 16)
- [ ] All four checkboxes ticked (financing, legal, license, notifications)
- [ ] Country: Portugal
- [ ] Profile URL pointing somewhere current
- [ ] Project repository URL pointing to the public Attestly repo
- [ ] Name field filled (full legal name on STF account)

---

*Form-answers v2 prepared 2026-05-26. Based on the actual form schema from view-source on apply.sovereigntechfund.de. If any field has an unexpected word limit, message the field name + limit and I'll re-compress in ~30 seconds.*
