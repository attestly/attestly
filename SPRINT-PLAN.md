# Attestly — Sprint Plan & Grant Submission

> **Primary funder**: **Sovereign Tech Fund** (rolling submission, €100k target ask).
> **Parallel funder**: **EIC Accelerator Step 1 → ANI Voucher Deep Tech** (€60k + €10k mechanical).
> **Deferred backup funder**: NLnet NGI0 Commons, 1 August 2026 cycle (after Conforme decision).
> **Goal**: Submit to STF by **Sat 31 May 2026** + file EIC Step 1 same week. Working PoC + 90-second demo + proposal in STF format.
>
> **Working directory**: `C:\claude\attestly\`
> **Status**: Pre-submission sprint, Day 0 (2026-05-21).
> **Revision**: rev 3 — STF-retargeted after confirming Conforme NLnet application went in last week (Cycle 13, decision ~late July). Filing Attestly to NLnet same cycle would force first-time-applicant overlap; STF removes that risk and lets us ask for €100k rather than €50k.

---

## 0. Headline

**Attestly is open, framework-agnostic verification infrastructure for EU AI Act Article 12 evidence.** It records AI decision events (CloudEvents standard) in a hash-chained append-only ledger and publishes Signed Tree Heads — cryptographic *commitments only* — to a public transparency log. Decision payloads remain private under operator control. The integrity of any decision can be independently verified by a regulator, civil society auditor, or affected individual without trusting the operator and without depending on any single commercial AI stack.

**Why now**: EU AI Act Article 12 enforcement begins **2 August 2026** (~10 weeks after our submission window). Article 12 mandates automatic logging over the lifetime of high-risk AI systems but is silent on tamper protection. Article 99 penalties: €15M / 3% global turnover.

**The Article 12 evidence gap**: Article 12 applies to all high-risk AI systems per Annex III (credit scoring, biometric, employment, education, essential services, law enforcement, migration, justice). The current generation of AI accountability tooling — including Microsoft's Agent Governance Toolkit and most observability stacks — covers *agentic* systems within their own controlled environment. **No open, neutral verification layer exists today for the broader population of high-risk AI systems, and none of the existing tooling produces evidence a regulator can verify independently of the operator's own infrastructure.** Attestly fills that gap as open digital base infrastructure.

---

## 1. Funder strategy — why STF first, not NLnet

NLnet NGI Zero Commons would have been a natural fit. Two reasons we're going to STF instead this cycle:

1. **Conforme is currently in NLnet evaluation** (submitted last week, Cycle 13, decision lands ~late July). NLnet explicitly advises first-time applicants to focus and offers "we may ask you to pick one" guidance when multiple proposals from the same applicant arrive in the same window. Submitting Attestly into the same cycle would put both proposals at risk.
2. **STF's ceiling is materially higher** (€50k minimum, no upper cap; real 2026 awards have ranged €50k-€1.28M). NLnet's first-time-applicant ceiling is €50k. We can scope Attestly for €100k under STF and fund Phase 1+2 in one ask.

The Aug-1 NLnet cycle remains a backup once the Conforme decision lands.

### The full 12-month funder plan

| Funder | Ask | When | Effort | Rationale |
|---|---|---|---|---|
| **Sovereign Tech Fund** | €100,000 | Submit by 31 May (rolling) | Full demo + proposal | Primary. €50k minimum, perfect thematic fit, decision in ~10 weeks |
| **EIC Accelerator Step 1** | (gate) | Submit same week | 5-page short form, no penalty for NO | If GO → unlocks ANI Voucher Deep Tech mechanically |
| **ANI Voucher Deep Tech** | €60k + €10k consult | After EIC Step 1 GO | File via ANI portal | Requires Lda; defer Lda 2 weeks |
| **NLnet NGI0** | €50,000 | Defer to 1 Oct 2026 cycle | Phase 1 builds the case | Wait until after Conforme decision (late July) + STF decision (late July) so we know what's needed |
| **OpenSSF Alpha-Omega** | $50-150k | Long shot, file when call opens | Lighter focus on supply-chain security than Article 12, but worth a parallel application | |

**Realistic 12-month capture**: €150k-€280k from STF + EIC/ANI alone, +€50k if NLnet Oct or OpenSSF lands.

---

## 2. The 90-second killer demo (working by Day 5)

Frame the demo around the **regulator's workflow**, not the engineering pipeline. STF reviewers want to see the public-interest value in the first 15 seconds.

```
$ # SCENE: A citizen has been denied credit. They file a complaint with the
$ # national AI authority. The regulator asks the operator for evidence on
$ # decision ID 23.

$ # Operator side — produce the regulator-facing evidence bundle
$ attestly export --decision-id 23 --for-regulator
[ok] Exported bundle: regulator-23.zip
     - decision payload (redacted per data-minimization policy)
     - inclusion proof for entry seq=23
     - reference to public Signed Tree Head: logs.attestly.dev/sth/2026-05-21T15:00Z

$ # Regulator side — verify the bundle independently
$ attestly verify --bundle regulator-23.zip
[ok] Decision seq=23 is included in the operator's published commitment.
     This is the decision record at the time of publication.

$ # — Now the operator quietly tampers with their database after the fact —
$ psql -c "UPDATE audit_log SET decision_outcome='approved' WHERE seq=23"
UPDATE 1

$ attestly export --decision-id 23 --for-regulator
$ attestly verify --bundle regulator-23.zip
[FAIL] Decision record for seq=23 no longer matches the published commitment.
       The operator's database has been altered after publication.
       Public commitment: hash=a1f2…  Current record: hash=9c4d…
```

That's the proof Article 12 needs. The log isn't proving the AI was *correct* — it's proving the operator can't *rewrite history*.

---

## 3. Decisions locked Day 0

| Decision | Choice | Rationale |
|---|---|---|
| Name | **Attestly** | Clean across GitHub/npm/PyPI/WHOIS (verified 2026-05-21). Evokes attestation. |
| Licence (code) | **Apache-2.0** | STF-friendly, patent grant, enterprise-friendly |
| Licence (spec) | **CC-BY-4.0** | STF-friendly |
| Tech stack — core | **Rust + `rs_merkle` (Phase 0)** | Minimal dependencies. Tessera is heavier than needed for MVP — deferred to Phase 2 as production backend. |
| Tech stack — SDK | **Python first (PyO3), TypeScript later (napi-rs)** | Python is the dominant high-risk AI language |
| DB | **Postgres 14+ + append-only triggers** | Universal install path |
| Identity | **`did:web` for orgs, Ed25519 keys for AI system instances** | Stable, standard, audit-friendly |
| Audit event format | **CloudEvents v1.0 (CNCF standard)** | Open envelope format Attestly can ingest from any AI system or framework. |
| Decision schema | **Attestly Decision Schema v0.1 — overlay on CloudEvents** | Vendor-neutral published spec. |
| Public log (MVP) | **JSON-file STH publication to a static host** | Cloudflare Pages / R2 / GitHub Pages — any static host. Tessera personality lands Phase 2. |
| Repo | `github.com/attestly/attestly` | Register Day 1 |
| Working dir | `C:\claude\attestly\` | |

---

## 4. STF thematic positioning (proposal narrative)

STF's stated mandate: **open digital base technologies, non-user-facing core technology**, strengthening digital sovereignty, resilience, and innovation. Past STF awards have funded curl, OpenSSL, Sequoia-PGP, WebAuthn documentation, Postgres-related infrastructure, Servo accessibility, KDE infrastructure.

**Our framing**:

> AI systems are increasingly making decisions about people — credit, hiring, healthcare, justice. The EU AI Act makes operators of high-risk AI systems legally accountable for these decisions. But Article 12 only mandates *logging* — it is silent on whether those logs can be trusted. Today, an operator can edit their own audit trail before a regulator inspects it.
>
> Attestly is open digital base infrastructure for AI accountability: a small, framework-agnostic library and verification protocol that gives high-risk AI operators a drop-in compliance layer whose integrity any third party — regulator, civil society auditor, affected citizen — can independently verify, without trusting the operator and without depending on any single commercial AI stack.
>
> Like Certificate Transparency for the web's PKI, Attestly publishes only cryptographic commitments to a transparency log. Decision payloads remain private under operator control. The infrastructure is small (a Rust core, language SDKs, a CLI verifier, a JSON wire format) and is designed to be one of several compatible implementations — not a platform.

**Why STF should care**: Article 12 enforcement begins 2 August 2026. Without open, neutral verification infrastructure, every high-risk AI operator in the EU will either depend on a single vendor's evidence claims (with no third-party verifiability), build something proprietary in-house (no commons), or — most likely — produce logs that nobody can trust if challenged. Attestly delivers the open base technology that lets EU AI Act enforcement actually work in practice.

---

## 5. Why this is commons infrastructure

Attestly is designed as **public digital infrastructure**, not as a startup product with an open-source layer on top. The schema, verifier, and transparency-log format are intended to be open, vendor-neutral, and usable by any party in the AI accountability ecosystem without dependence on a single commercial stack.

- **Regulators** can independently verify that Article 12 evidence was not altered after the fact, using a CLI or web verifier that does not require operator cooperation.
- **Operators** of high-risk AI systems get a drop-in integrity layer without changing their model frameworks or pipelines — Attestly ingests events from any source that can emit CloudEvents.
- **Civil society and affected individuals** gain a path to inspect published commitments instead of relying solely on operator assertions or aggregate reports.
- **Open-source ecosystems** can build compatible tooling — alternative verifiers, language-specific SDKs, sectoral plugins, regulator-specific UIs — around a published evidence and verification standard.

The reference precedent is Certificate Transparency: a published commons protocol with multiple compatible implementations, not a proprietary platform.

---

## 6. Privacy posture (must be on proposal page 1)

A reviewer's most likely first-page question: *"are you publishing sensitive AI decisions to a public log?"* The answer is **no**:

- **Only hashes, commitments, and Signed Tree Heads are published publicly.** No decision payload, no personal data, no model inputs leave the operator's environment by default.
- **Decision payloads remain under operator control** in the operator's own Postgres database. They are exported selectively, by the operator, only when needed for an audit, dispute, or individual subject-access request.
- **The schema and SDK document data-minimization expectations** so operators do not place personal data into public artifacts. Hashes are computed over canonicalized payloads on-prem; only the resulting digest crosses the public boundary.
- **GDPR alignment**: hashes published to a transparency log are not personal data in the GDPR sense, because they are cryptographic digests that do not permit reconstruction of the underlying decision without operator cooperation.

This separation — *commitments public, payloads private* — is the same pattern Certificate Transparency uses for certificates, and Sigstore uses for software-supply-chain attestations. It is regulator-friendly and GDPR-compatible by construction.

---

## 7. Governance and neutrality

Attestly is published as an open commons from day one and is explicitly not a single-company internal tool released as open source.

- **Public repository and open specification from the start.** No private working code; everything in `github.com/attestly` from Day 1.
- **Neutral naming and stewardship of the verification format.** The Decision Schema and STH format are versioned and published independently of any single operator or vendor.
- **External pilot users and contributors welcome during the evaluation period.** Conforme is one of several intended pilots, not the host project.
- **Separation between commons infrastructure and any future commercial services.** The library, spec, CLI, and verifier are Apache-2.0 + CC-BY-4.0 in perpetuity. Any future commercial offerings (hosted log, managed compliance, support) are built *on top of* — never inside — the commons artifacts.

---

## 8. Architecture (1-page diagram for proposal)

```
        ┌─────────────────────────────────────────┐
        │   Any high-risk AI system               │
        │   (credit scoring, hiring AI, medical   │
        │   diagnostic, biometric, agent stack)   │
        └─────────────────┬───────────────────────┘
                          │ emits CloudEvents v1.0
                          │ (decision + inputs + model_id + context)
                          ▼
        ┌─────────────────────────────────────────┐
        │   Attestly SDK (Python / TS)            │
        │   - sign event with system instance key │
        │   - decorate with Decision Schema v0.1  │
        │   - append to local Postgres ledger     │
        └─────────────────┬───────────────────────┘
                          │   ┌─ payloads stay private here ─┐
              ┌───────────▼───┴──────────┐
              │ Postgres audit_log table │
              │ (append-only ledger,     │
              │  operator-controlled)    │
              └───────────┬──────────────┘
                          │ batched: hash only, never payload
                          ▼
        ┌─────────────────────────────────────────┐
        │   Attestly Commitment Publisher         │
        │   - compute Merkle root over hashes     │
        │   - sign with org identity key          │
        │   - publish Signed Tree Head            │
        └─────────────────┬───────────────────────┘
                          │   ── PUBLIC BOUNDARY ──
                          ▼   (only hashes and signatures cross)
        ┌─────────────────────────────────────────┐
        │   Public transparency log               │
        │   - signed checkpoints only             │
        │   - any party can fetch + verify        │
        │   - no decision data exposed            │
        └─────────────────────────────────────────┘
                          │
        ┌─────────────────▼───────────────────────┐
        │   Public Verifier                       │
        │   (CLI + WASM web UI, no DB access)     │
        │   - regulator / citizen runs it locally │
        │   - verifies an exported decision       │
        │     bundle against the public log       │
        └─────────────────────────────────────────┘
```

---

## 9. Phase 0 — Pre-submission sprint (Day 1 → Day 10)

Items marked **[CRITICAL]** must exist for the submission. Items marked **[STRETCH]** improve polish but can be cut if behind schedule.

### Day 1 (Thu 2026-05-22) — Setup
- [ ] **[CRITICAL]** Register GitHub org `attestly`, reserve PyPI `attestly`, npm `@attestly/sdk`
- [ ] **[CRITICAL]** Register domain `attestly.dev`
- [ ] **[CRITICAL]** Create Sovereign Tech Fund applicant account at https://www.sovereign.tech/programs/fund and read submission criteria + timeline
- [ ] **[CRITICAL]** Create EU Funding & Tenders Portal account at https://ec.europa.eu/info/funding-tenders/opportunities/portal/ to enable EIC Accelerator Step 1 submission
- [ ] **[CRITICAL]** Initialize Cargo workspace + Python skeleton in `C:\claude\attestly\`; add `LICENSE-APACHE`, `LICENSE-CC-BY`, `README.md` (with the moat sentence as line 1)
- [ ] **[STRETCH]** Set up minimal `attestly.dev` landing page (single static HTML, says "open accountability infrastructure for EU AI Act Article 12, STF-funding-in-progress")

### Day 2 (Fri 2026-05-23) — Core ingest
- [ ] **[CRITICAL]** Postgres schema + append-only triggers (see CODING-SPRINT.md §7)
- [ ] **[CRITICAL]** Rust `attestly-core::ledger::append` + PyO3 wrapper
- [ ] **[CRITICAL]** Integration test: append → query → verify all present

### Day 3 (Sat 2026-05-24) — Merkle commitments + signed checkpoint
- [ ] **[CRITICAL]** `rs_merkle`-backed tree + root computation
- [ ] **[CRITICAL]** SignedCheckpoint generation + JSON serialization
- [ ] **[CRITICAL]** `attestly publish-checkpoint --target file:///...` works
- [ ] **[CRITICAL]** Property tests at 5000 iterations pass

### Day 4 (Sun 2026-05-25) — Verifier CLI + regulator-export workflow
- [ ] **[CRITICAL]** `attestly export --decision-id <id> --for-regulator` produces `regulator-N.zip` bundle
- [ ] **[CRITICAL]** `attestly verify --bundle <zip>` round-trips on a clean bundle
- [ ] **[CRITICAL]** End-to-end smoke test: export → verify → tamper Postgres → verify fails

### Day 5 (Mon 2026-05-26) — The killer demo
- [ ] **[CRITICAL]** `examples/credit_score_decision.py` — 50 synthetic credit-scoring decisions via Python SDK
- [ ] **[CRITICAL]** Record the 90s demo screencast framed around the regulator workflow (see §2)
- [ ] **Day 5 fallback plan**: if the full demo is not compelling by EOD, freeze engineering scope, preserve the strongest working path, and spend remaining time on proposal quality, narrative clarity, and a roadmap that shows how the demo reaches production shape during the evaluation period. Submit a written-strong proposal with the partial demo rather than no proposal at all.

### Day 6 (Tue 2026-05-27) — STF proposal draft 1 + EIC Step 1 draft
- [ ] **[CRITICAL]** Write STF proposal in their submission portal format. STF asks for:
  - Project description (focus on open digital base technology + AI accountability)
  - Technical approach + deliverables
  - Budget breakdown (€100k — see §13)
  - Team + governance + sustainability
  - Why this work is essential to digital sovereignty / EU AI Act enforcement
- [ ] **[CRITICAL]** Draft EIC Accelerator Step 1 short application (~5 pages). Same product, EIC framing: "novel deep-tech infrastructure for EU AI Act compliance verification". Worst case: NO with feedback, no penalty.

### Day 7 (Wed 2026-05-28) — Letters of support (optional for STF) + EIC Step 1 polish
- [ ] **[STRETCH]** Conforme commercial-backer letter (1pp) — useful but not required by STF
- [ ] **[STRETCH]** Cold-email Fraunhofer SIT / INESC TEC for academic letter
- [ ] **[STRETCH]** Cold-email AlgorithmWatch / EDRi for civil society letter
- [ ] **[CRITICAL]** EIC Step 1: polish based on Day 6 draft; verify it stands alone (EIC's 5-page form is judged on its own merits, no LOIs required for Step 1)
- [ ] **[STRETCH]** Public `pre-launch` GitHub README + a short blog post on conforme.info seeding an "early backers" page

### Day 8 (Thu 2026-05-29) — Proposal review
- [ ] **[CRITICAL]** Send STF draft to 2 external reviewers (Conforme network: regulatory-adjacent + technical)
- [ ] **[CRITICAL]** Send EIC Step 1 draft to 1 reviewer with EIC experience (worth tracking down via LinkedIn)
- [ ] **[CRITICAL]** Apply feedback

### Day 9 (Fri 2026-05-30) — Polish + final demo cut
- [ ] **[CRITICAL]** Re-record demo with final names/branding if Day 5 was rough
- [ ] **[CRITICAL]** Re-read both proposals aloud; tighten anything over-long
- [ ] **[CRITICAL]** Verify all URLs work, all licences in place, all repos public

### Day 10 (Sat 2026-05-31) — Submit STF + EIC Step 1
- [ ] **[CRITICAL]** Submit STF via https://www.sovereign.tech/programs/fund (rolling, no fixed deadline — but ship Saturday to lock the work)
- [ ] **[CRITICAL]** Submit EIC Accelerator Step 1 via https://ec.europa.eu/info/funding-tenders/opportunities/portal/ before the next monthly cut-off
- [ ] **[CRITICAL]** Confirm receipt emails for both

---

## 10. Phase 1 — Evaluation period (June → late July, ~8 weeks)

STF's decision timeline is up to 10 weeks. EIC Step 1 monthly batches resolve faster (~4-6 weeks). Use the window:

| Week | Focus | Output |
|---|---|---|
| W1-2 | Polish core + Python SDK | v0.2 published to PyPI |
| W3-4 | Spec v0.1 — Decision Schema, checkpoint format, verifier protocol | `spec/` markdown + JSON Schemas published |
| W5-6 | First pilot integrations | Conforme NRUA decisions logged to Attestly. Outreach to one additional non-Conforme pilot for vendor-neutrality proof. |
| W7-8 | EIC Step 1 decision lands; respond | If GO: file ANI Voucher Deep Tech application via ANI portal (requires Lda — start incorporation now if not done). If NO: incorporate feedback into next EIC cycle. |

**Lda decision point**: trigger Lda incorporation (Sociedade Unipessoal por Quotas via empresaonline.pt, ~€500, 1-2 weeks) the moment EIC Step 1 returns GO. Don't pre-pay; ANI Voucher requires Lda but EIC Step 1 doesn't.

If STF rejects, Phase 1 still produces a working OSS library + at least one named downstream user. Either re-apply to STF (different scope), file NLnet 1 Oct cycle, or apply OpenSSF/Mozilla MTF when their windows open.

---

## 11. Phase 2 — Funded build (Aug → mid-October, 10-12 weeks)

Triggered if STF approves. Scope is broader than the original NLnet plan because STF funds €100k vs NLnet's €50k cap.

| Week | Goal | Deliverable |
|---|---|---|
| W7-8 | TypeScript SDK | `@attestly/sdk` on npm via napi-rs |
| W9 | Production transparency-log integration | Migrate from MVP file-based STHs to Tessera personality with proper inclusion + consistency proofs |
| W10 | Web verifier (WASM, no JS framework) | `verify.attestly.dev` public UI |
| W11 | Article 12 export pack | `attestly export --article-12` produces regulator-ready bundle with full provenance chain |
| W12 | Docs site + 3 worked examples | `docs.attestly.dev` with credit scoring, employment screening, and an "ingest from Microsoft AGT" bridge |
| W13 | Security review prep | `cargo audit`, `pip-audit`, threat model doc, submit to Radically Open Security |
| W14 | AGT bridge adapter | Ingest CloudEvents from Microsoft AGT → Attestly Decision Schema as one interop example |
| W15-16 | Standards engagement | Submit Decision Schema to CNCF or W3C Community Group; participate in W3C Agent Identity Registry Protocol CG |
| W17-18 | v1.0 release + announcement | crates.io + PyPI + npm; Hacker News, Lobsters, EU AI Act subreddit; FOSDEM 2027 submission |

**Concurrent with Phase 2**: prepare **NLnet NGI0 Commons application for 1 October cycle** as follow-on funding (Phase 3 scope: federation, EUDI Wallet binding, selective disclosure). By Oct: Conforme NLnet decision is known, Attestly STF decision is known, Attestly v1.0 has shipped or is shipping. Strong follow-on case.

---

## 12. The killer demo evolution

| Day / Week | What it shows |
|---|---|
| 2 | Postgres trigger blocks UPDATE/DELETE |
| 3 | Merkle root recomputes consistently across 100 appends |
| 4 | `attestly export --for-regulator` + `attestly verify --bundle` round trip |
| 5 | **PUBLIC v0**: 90s regulator-workflow screencast — disputed decision → export → verify → tamper → verify fails |
| W3 | Polished version with proper credit-scoring AI simulation |
| W6 | Conforme's NRUA wizard producing real Article 12 evidence |
| W10 | Web verifier UI — regulator drops a bundle file, gets verdict |
| W18 | Full pitch reel — 90 seconds covering moat, demo, compliance mapping, multi-source story (standalone + AGT bridge + custom ML pipeline) |

---

## 13. STF budget breakdown (€100,000 target)

STF's minimum is €50k and they reward larger scope that delivers real infrastructure. Same code base as the original €50k plan, more deliverables funded:

| Line | Amount | Notes |
|---|---|---|
| Core engineering (12 weeks of Phase 0-1 build) | €60,000 | Rust core + Python SDK + verifier CLI + Conforme pilot integration |
| TypeScript SDK + WASM web verifier (Phase 2) | €15,000 | napi-rs build, browser verifier UI |
| Spec writing + standards-body engagement | €8,000 | Decision Schema v1.0 spec, checkpoint format spec, verifier protocol spec, Article 12 mapping doc, CNCF / W3C CG submission |
| Community / governance / maintainer coordination | €4,000 | RFC process, contributor onboarding, pilot-user coordination |
| Production transparency-log infrastructure | €3,000 | Tessera-backed hosted log + static hosting + CDN for verifier UI |
| Translation (PT + ES + DE + FR) of regulator-facing UI | €3,000 | Member-state authorities will read these |
| Independent security review (Radically Open Security) | €5,000 | STF typically funds review separately; budget defensively |
| Travel (FOSDEM 2027 + Article 12 community events) | €2,000 | |
| **Total** | **€100,000** | |

STF pays milestone-based after scoping phase. Typical schedule (to be confirmed in their scoping call):
- M1 (end W8): core + Python SDK on PyPI — 25%
- M2 (end W12): public verifier live + production transparency-log integration — 25%
- M3 (end W16): Article 12 export pack + security review complete — 25%
- M4 (end W18): v1.0 + docs + 3 worked examples — 25%

---

## 14. Risk register

### Technical
| Risk | Sev | Mitigation |
|---|---|---|
| Microsoft AGT extends to non-agentic high-risk AI | MED | Our format is open + neutral and ingests CloudEvents from any source. AGT's lack of stable schema commitment helps us; we frame as the independent verification layer regardless of which framework produced the events. |
| Performance overhead > 5ms per event | LOW | Hash + sign + append is fast. Benchmark Day 3, publish numbers. |
| CloudEvents schema drift | LOW | We define our own Decision Schema overlay; CloudEvents is just the envelope. |
| Tessera Postgres backend Alpha at integration time | LOW | Phase 0 doesn't depend on Tessera. Phase 2 has full evaluation period to assess; fallback is filesystem-backed checkpoint publication. |

### Market
| Risk | Sev | Mitigation |
|---|---|---|
| EU AI Act enforcement delayed | LOW | Aug 2 2026 is in legislation, won't slip. Even if national authorities are slow, the regulation is binding. |
| STF funds a competitor mid-build | MED | STF tends to fund distinct projects; we have a clear differentiator (Article 12 evidence integrity vs supply-chain security). Watch for new STF awards in the AI accountability space. |

### Compliance / perception
| Risk | Sev | Mitigation |
|---|---|---|
| Public transparency perceived as incompatible with GDPR | MED | Privacy posture front-and-centre (§6). Only hashes and signed checkpoints public; payloads remain operator-controlled. Certificate Transparency + Sigstore precedent. |
| The project appears too tied to Conforme | MED | Governance + neutrality language (§7). At least one additional non-Conforme pilot user named during evaluation. Spec and verifier vendor-neutral from Day 1. |

### Grant
| Risk | Plan B |
|---|---|
| STF rejection | Re-apply STF with refined scope (rolling — no cooldown). File NLnet 1 Oct cycle when Conforme decision is known. Apply OpenSSF Alpha-Omega when their next call opens. |
| EIC Step 1 NO | No penalty, file feedback into next month's batch. ANI Voucher path delayed but not killed. |
| Conforme NLnet rejection (lands ~late July) | Frees up the NLnet Aug or Oct cycle for Attestly as fresh first-time proposal. Net positive for Attestly NLnet path. |
| Conforme NLnet approval | Returning-applicant status helps. Email NLnet first before submitting Attestly to confirm welcome. |
| Day 5 demo not compelling | Apply Day 5 fallback plan (§9): preserve strongest working path, prioritise written-strong proposal + roadmap. Submit anyway. STF in particular weights written technical depth heavily. |
| Conforme firefight eats the 10 days | Hard rule: Conforme P0 incidents only. P1+ deferred to post-31-May. |

---

## 15. Open questions for the proposal-facing plan

- [ ] **Additional pilot user**: who beyond Conforme can we name as an intended pilot for vendor-neutrality? Identify by Day 7.
- [ ] **Standards strategy**: should the Decision Schema be submitted to CNCF (CloudEvents WG) or W3C CG? Decision can wait until W3.
- [ ] **EIC Step 1 cut-off**: confirm next monthly batch date. EIC Step 1 has monthly batches; Day 10 submission targets the next available batch.
- [ ] **STF scoping-phase prep**: STF doesn't accept supplemental materials; everything must be in the form. Be sure the form's narrative fields are tight enough to stand alone.

## 15b. Internal questions (not for proposal)

- Lda decision: defer incorporation until EIC Step 1 returns GO (~6 weeks after submission). Save €500 + accountant fees if NO. If GO: incorporate immediately for ANI Voucher.
- Conforme bandwidth: confirm 60% capacity for next 10 days. If Conforme P0 fires, this slips a week.

---

## 16. Pre-Day-1 setup checklist (do tonight)

- [ ] Read this doc end to end, push back on anything that's wrong
- [ ] Decide: 60% bandwidth commit for 10 days? Yes/no
- [ ] Reserve the deadline in calendar: Sat 2026-05-31 (STF + EIC Step 1 submission target)
- [ ] Confirm name: Attestly OR alternative (1 more day window to change)
- [ ] Confirm strategy: STF primary + EIC Step 1 parallel + NLnet Oct deferred backup

---

## 17. Reference URLs

### Primary (STF)
- [Sovereign Tech Fund — apply](https://www.sovereign.tech/programs/fund) — rolling, application portal open
- [STF submission criteria + process](https://www.sovereign.tech/news/new-proposals-criteria-process-timeline)
- [STF Resilience Program](https://www.sovereign.tech/news/call-for-tenders-resilience) — CRA-aligned, additional context

### Parallel (EIC Step 1 → ANI Voucher Deep Tech)
- [EU Funding & Tenders Portal](https://ec.europa.eu/info/funding-tenders/opportunities/portal/) — EIC Accelerator submission
- [EIC Accelerator 2026 guide](https://eic.ec.europa.eu/eic-funding-opportunities/eic-accelerator_en)
- [ANI Voucher Deep Tech](https://ani.pt/concursos/voucher-deep-tech-avanco-na-maturidade-tecnologica/) — €60k + €10k, triggered by EIC Step 1 GO

### Deferred backup (NLnet Aug or Oct cycle)
- [NLnet Commons Fund](https://nlnet.nl/commonsfund/) — apply via https://nlnet.nl/propose
- [NLnet Guide for Applicants](https://nlnet.nl/commonsfund/guideforapplicants/)
- [NLnet FAQ](https://nlnet.nl/commonsfund/faq/) — first-time-applicant guidance, parallel-submission policy

### Long-shot
- [OpenSSF Alpha-Omega](https://alpha-omega.dev/) — security infrastructure, $50-150k typical
- [Mozilla Foundation grantmaking](https://www.mozillafoundation.org/en/what-we-fund/) — watch for next MTF cycle (late 2026 expected)

### Technical references
- [EU AI Act Article 12](https://artificialintelligenceact.eu/article/12/)
- [Certificate Transparency (RFC 6962)](https://www.rfc-editor.org/rfc/rfc6962) — model for commitments-not-payloads pattern
- [Sigstore Rekor](https://github.com/sigstore/rekor) — transparency-log reference architecture
- [transparency-dev Tessera](https://github.com/transparency-dev/tessera) — Phase 2 backend candidate
- [CloudEvents v1.0 spec](https://github.com/cloudevents/spec) — event envelope standard
- [Microsoft AGT audit + compliance docs](https://microsoft.github.io/agent-governance-toolkit/tutorials/04-audit-and-compliance/) — one possible event source for Attestly to ingest
- [Auditable Agents paper](https://arxiv.org/pdf/2604.05485) — academic foundation to cite

---

*End of plan. 10 days. Working demo by Day 5. Submit STF + EIC Step 1 by Day 10. NLnet Oct cycle deferred. Realistic 12-month non-repayable capture: €150-280k.*
