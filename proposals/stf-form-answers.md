# STF Application Form — Pre-Compressed Answers

> Companion to `stf-proposal.md`. The long-form proposal lives there;
> this file holds compressed versions sized for the apply.sovereigntechfund.de
> structured form fields. Word counts noted per section. If the actual form
> has a different limit, message me and I'll re-compress.

---

## Project title (form expects ≤ 1 line)

```
Attestly
```

May be identical with application name (per form hint). If the field permits a sub-title:

```
Attestly — Open verification infrastructure for EU AI Act Article 12 evidence
```

---

## Describe your project in a sentence (max 100 words)

Attestly is an open-source library that gives any high-risk AI system a tamper-evident audit log. It records each decision in an append-only, cryptographically-signed ledger; publishes only Merkle-rooted commitments to a public transparency log; and exports evidence bundles a regulator can verify locally with a standalone tool. The architecture is Certificate Transparency for AI accountability — public commitments, private payloads, GDPR-compatible by construction. Apache-2.0 in perpetuity, framework-agnostic, no commercial dependency. Designed to satisfy EU AI Act Article 12 (enforceable August 2026) and eIDAS 2 Article 45l Qualified Electronic Ledger requirements (December 2026).

**~95 words.**

---

## Describe your project more in-depth. Why is it critical? (max 300 words)

From 2 August 2026, EU AI Act Article 12 makes automatic decision-logging mandatory for every high-risk AI system — credit scoring, hiring, biometric identification, employment, law enforcement, migration, judicial decisions. Non-compliance reaches €15M or 3% of global turnover. The Act tells operators what to log but not how to make those logs trustworthy.

Today an operator can edit their own audit trail before a regulator inspects it, and no off-the-shelf technology lets the regulator detect the edit. This is the integrity gap the law leaves open. The closest existing tool — Microsoft's Agent Governance Toolkit (April 2026) — scopes only to agentic AI within Microsoft's framework stack, and provides only in-process verification (the regulator must trust the operator's infrastructure). Content-provenance standard C2PA has a documented timestamp-substitution flaw (arXiv 2604.24890) that explicitly excludes it from legal-evidence use cases.

Attestly closes the gap with three composable primitives: an append-only signed event ledger; publicly-published Merkle-rooted Signed Tree Heads (decisions stay private; only short commitments touch public infrastructure); and a regulator-runnable standalone verifier requiring no operator cooperation. The architecture is Certificate Transparency for AI accountability — a pattern with thirteen years of production use in the TLS ecosystem.

Critical now because two EU regulatory deadlines converge: AI Act Article 12 enforceable August 2026; eIDAS 2 Regulation 2024/1183 Article 45l recognises Qualified Electronic Ledgers with legal presumption of authenticity from December 2026 — the first time EU regulatory framework names cryptographic ledgers as evidentiary infrastructure. Attestly is the open reference implementation that bridges both. ENISA preparatory analysis estimates Article 12 compliance costs at €0.5–10M per high-risk system; a standardised open primitive reduces this from per-operator custom build to drop-in dependency.

**~295 words.**

---

## Why your team? (likely ~200 words)

Curtis (Portugal-based founder, sole technical lead): 15+ years software engineering across regulated industries. Built and ships **Conforme** (https://conforme.info), a live EU short-term-rental compliance SaaS in production with paying PT customers, integrating with the Portuguese tax authority, border police, and tourism registry. Demonstrated solo end-to-end delivery on regulated-industry software under multiple overlapping legal frameworks.

Attestly emerges from the observation, made during Conforme's compliance work, that Article 12 mandates audit logging but is silent on integrity — and that the same gap exists in every high-stakes domain where one party both produces and stores evidence about its own conduct. Conforme's NRUA registration wizard is the natural first pilot (it emits structured decision events that are Article-12-shaped).

Co-maintainer to be invited by M2 to satisfy the multi-maintainer neutrality criterion; targets identified are EU-resident contributors with prior `spruceid/ssi`, Sigstore, or `transparency-dev` experience.

Advisory pilot relationships in flight: Horizontal (Tella maintainers — civil-society capture-tool); International Bar Association eyeWitness team; INESC TEC CRACS (cryptographic-protocol review, outreach 2026-05-26).

**~185 words.**

---

## Project scope / what will you deliver? (likely ~300-400 words; can be a structured milestone list)

Phase 1 takes Attestly from v0.1 (working PoC, 40+ tests, demo screencast) to v1.0 (production-grade library, web verifier, security audit, standards engagement). 16 weeks, four milestones.

**M1 (week 4) — Production-quality core + Python SDK**
- Rust workspace passes `cargo audit` + `cargo deny` clean
- Performance baseline published: ≤ 2 ms median append, ≤ 10 ms tree root over 10k leaves
- Python SDK (`attestly` on PyPI) with PyO3 bindings + adapter framework
- Spec v0.1 frozen and published at `spec.attestly.org`
- First non-Conforme pilot user named publicly

**M2 (week 8) — Production transparency log + browser WASM verifier**
- Migration from MVP file-based STHs to Tessera (`transparency-dev`) Postgres-backed personality
- `logs.attestly.org` live, serving signed checkpoints and inclusion proofs over HTTPS
- `verify.attestly.org` — 200 kB WASM artefact loaded by a vanilla HTML page; drag-and-drop a regulator bundle, get a verdict in the browser
- TypeScript SDK published to npm

**M3 (week 12) — Article 12 evidence pack + independent security review**
- `attestly export --article-12` produces a regulator-ready bundle covering an arbitrary date range with a README cross-mapping every Article 12 sub-requirement to a concrete artefact
- Independent security review by Radically Open Security (or equivalent) commissioned at start of M2, report received end of M3, blocker findings resolved
- Compliance mapping documents: Article 12, eIDAS 2 Art 45l, SOC 2, GDPR

**M4 (week 16) — v1.0 + standards engagement**
- v1.0.0 tagged and published to crates.io + PyPI + npm
- Microsoft AGT bridge adapter shipped (ingests AGT CloudEvents into Attestly format) — proves multi-vendor neutrality
- Decision Schema submitted to CNCF CloudEvents WG or W3C Community Group
- IETF SCITT WG participation: position document or Internet-Draft mapping Attestly's transparency-log model onto SCITT receipt patterns
- FOSDEM 2027 + IIW Spring 2027 talk submissions filed

Each milestone unblocks 25% of the grant disbursement.

**~365 words.**

---

## Why does this need public funding? (likely ~200 words)

Attestly's deliverable is base infrastructure that every EU high-risk AI operator can use without licensing or vendor lock-in. The commercial model that funds proprietary equivalents — pay-per-seat SaaS, vendor-stack lock-in, opaque enterprise contracts — is structurally incompatible with the trust assumption a public-anchor primitive requires. A regulator must be able to verify evidence independent of any vendor's incentives.

This is the same logic the Sovereign Tech Fund applies to `curl` (network base) and OpenSSL (crypto base): a small, reusable, open-source library that an entire ecosystem depends on but that no single market actor has commercial incentive to build openly. AI accountability infrastructure is the same shape of problem on a one-year timescale.

The applicant maintains a commercial product (Conforme) that provides primary income, decoupling Attestly's continued maintenance from grant cycles. The grant pays for the **open-source library, specifications, web verifier, and standards engagement** that will not be built by venture-funded commercial alternatives — none of which would publish their integrity primitives under Apache-2.0 in perpetuity. A separate proprietary hosted-dashboard layer, funded by parallel commercial channels (EIC Accelerator Step 1 → ANI Voucher), sits on top of the open library without duplicating it.

**~200 words.**

---

## Open-source status (likely ≤ 100 words / structured fields)

- **Code licence**: Apache-2.0 in perpetuity (Rust core, CLI, verifier, Python + TypeScript SDKs)
- **Spec licence**: CC-BY-4.0 in perpetuity (Decision Schema, Checkpoint Format, Verifier Protocol)
- **Repository**: https://github.com/attestly/attestly (public from day 1)
- **Governance**: BDFL pattern documented in `GOVERNANCE.md`; one additional co-maintainer invited by M2 to satisfy multi-maintainer neutrality
- **Commercial layer**: a separate proprietary hosted-dashboard sits *on top of* the open library, in a separate repository, never inside grant deliverables

---

## Budget breakdown (€100,000 total — usually structured numeric fields)

| Line | Amount |
|---|---|
| Core engineering — Rust core, Python SDK, verifier CLI, first pilot integration (12 wks @ €5,000) | €60,000 |
| Integration & release engineering — TypeScript SDK, WASM verifier, multi-platform CI, package publication | €15,000 |
| Specifications + standards-body engagement — Decision Schema v1.0, Checkpoint Format, Verifier Protocol, Art 12 + eIDAS 2 Art 45l mapping docs, CNCF/W3C submission | €8,000 |
| Community + governance + maintainer coordination | €4,000 |
| Production transparency-log infrastructure — Tessera-backed hosted log + CDN | €3,000 |
| Translation of regulator-facing UI (PT + ES + DE + FR) | €3,000 |
| Independent security review (Radically Open Security or equivalent) | €5,000 |
| Travel — FOSDEM 2027 (Brussels) + Article 12 / regulator engagement events | €2,000 |
| **Total** | **€100,000** |

All amounts VAT-exclusive (Portugal-based applicant; VAT treatment per Sovereign Tech Fund's standard scoping process).

---

## Schedule (likely ≤ 100 words)

16 weeks from grant acceptance (post-scoping call). Four 4-week milestone segments (M1–M4 above). Lead developer at 60% allocation; Conforme (existing commercial product) provides remaining 40% of time. Buffer of ~10% built into M3 to absorb security-review feedback loop. Phase 1 completes by week 16 with v1.0 tagged, security review resolved, and standards-body submissions filed.

**~80 words.**

---

## Risk + mitigation (likely ~200 words)

**1. Single-lead-developer key-person risk.** Mitigation: multi-maintainer onboarding by M2 (governance-criterion satisfaction); project artefacts open-source from day 1, so continuity is not vendor-locked.

**2. Article 12 enforcement guidance shifts during the grant period.** Mitigation: Article 12 mapping document (M3 deliverable) is versioned and updates incrementally; the underlying primitive (Merkle-anchored signed event ledger) is regulation-agnostic.

**3. Tessera production-log integration encounters Postgres-personality stability issues.** Mitigation: MVP file-based STHs ship in M1 and remain a supported fallback; the M2 Tessera migration is a quality-of-service upgrade, not a hard dependency.

**4. Independent security review finds critical findings late in M3.** Mitigation: 10% buffer in M3 schedule; review commissioned at M2 start (not M3) to leave at least 4 weeks for remediation.

**5. Standards-body engagement (CNCF / W3C / IETF SCITT) is slower than Phase 1 timeline.** Mitigation: M4 deliverable is the *submission* + acknowledged receipt, not full WG adoption. WG adoption is downstream and explicitly out of grant scope.

**~195 words.**

---

## Why Sovereign Tech Fund specifically? (likely ~150 words)

Sovereign Tech Fund funds open digital base technologies — the kind of small, reusable, framework-agnostic libraries that ecosystems depend on but that no commercial actor has incentive to build openly. STF's existing portfolio (curl, OpenSSL, KDE, Postgres infrastructure, Sequoia-PQ) is exactly the shape of project Attestly is. AI accountability infrastructure is the same problem on a one-year timescale.

STF specifically (vs other open-source funders) because:

1. The funding band (€50k–€200k) matches the Phase 1 scope without forcing artificial scope-up.
2. STF accepts rolling submissions and evaluates on technical merit, not on quarterly competitive cycles — appropriate for infrastructure work.
3. STF's bug-bounty and security-review programmes are natural M2-M3 enablers (independent review explicitly budgeted, with STF's own review programme as a complementary channel).
4. STF's track record of funding base-layer EU digital infrastructure positions Attestly inside a portfolio narrative that EU regulators are increasingly aware of.

**~150 words.**

---

## Sustainability beyond the grant (likely ~200 words)

Library, CLI, verifier, and specifications remain Apache-2.0 + CC-BY-4.0 in perpetuity. Post-grant maintenance is funded through four independent channels:

1. **Conforme commercial revenue** — applicant's existing SaaS product (paying PT customers) provides primary income, decoupling Attestly's maintenance from grant cycles.
2. **A separate hosted-dashboard service** built on top of the open commons artefacts. Open core remains free; hosted multi-system management dashboard is the commercial sustainability layer (AGPL-3.0 + proprietary hosted SaaS, never importing private functionality back into the open libraries).
3. **Parallel grant streams for adjacent scope**:
   - Open Technology Fund Internet Freedom Fund (concept note submitted) — civil-society integration scope
   - NLnet NGI Zero Commons October 2026 cycle (in preparation) — Phase 3 federated multi-org logs + EUDI Wallet binding
   - Internet Society Foundation Research Grant 2026 (in preparation) — empirical study scope
4. **EIC Accelerator Step 1 + ANI Voucher Deep Tech (€70k mechanical on GO)** — funds the proprietary commercial layer that sits on top of the open library, not the library itself.

No deliverable is funded twice; non-overlap statement covers each funder.

**~195 words.**

---

## Parallel funding / non-overlap (form may ask explicitly)

Parallel applications are in flight or in preparation:

- **OTF Internet Freedom Fund** (concept note submitted): civil-society and journalism evidence-integrity scope — Tella integration adapter, browser WASM verifier with multilingual UI, IETF SCITT spec submission, IBA partnership. None of these deliverables appear in this STF list.
- **NLnet NGI Zero Commons, October 2026 cycle** (in preparation): Phase 3 — federated multi-organisation transparency logs and EUDI Wallet operator-identity binding. Out of STF Phase 1 scope.
- **ISF Research Grant 2026** (in preparation): empirical study scope — peer-reviewed analysis. Research outputs, not engineering deliverables.
- **EIC Accelerator Step 1** (in preparation): unlocks **ANI Voucher Deep Tech (€60k + €10k)** mechanically on GO; funds a proprietary hosted-dashboard layer **on top of** the open library, never duplicating it.

No deliverable is funded twice. Applicant will withdraw any application or rescope any deliverable that STF identifies as overlapping with another funded grant.

**~145 words.**

---

## Demo / proof-of-concept (form likely has a URL field)

- **Public demo**: https://attestly.org (27-second tamper-detection screencast embedded)
- **Source repository**: https://github.com/attestly/attestly
- **Reproducible end-to-end demo**: `git clone … && cargo build --release && bash examples/demo.sh` (~5 min)
- **Test count**: 40+ across the workspace (20 in `attestly-core`, 20 in `attestly-tella` scaffold)

---

## Contact

- **hello@attestly.org** — preferred for project correspondence
- **curts152@gmail.com** — backup
- **Project URL**: https://attestly.org
- **Repository**: https://github.com/attestly/attestly

---

*Form-answers file prepared 2026-05-26 as companion to stf-proposal.md v2. If a form field has a word-count limit the answer above exceeds, message the limit and the exact field name and I'll re-compress.*
