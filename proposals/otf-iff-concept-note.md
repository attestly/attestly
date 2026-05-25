# OTF Internet Freedom Fund — Concept Note

**Project**: Attestly — A public-anchor cryptographic integrity layer for journalism and human-rights field evidence
**Funder**: Open Technology Fund — Internet Freedom Fund
**Funder portal**: https://www.opentech.fund/funds/internet-freedom-fund/
**Submission target**: rolling, monthly review
**Funding ask**: **US $120,000** over 12 months
**Lead applicant**: Curtis (Portugal-based, sole technical lead)
**Project repository**: https://github.com/attestly/attestly (public, Apache-2.0)
**Project website**: https://attestly.org

---

## Tagline (≤ 200 chars)

An open, public-anchor cryptographic integrity layer that field-capture tools can adopt to strengthen the evidentiary value of journalism and human-rights documentation against later authenticity disputes.

---

## Project Summary (≤ 250 words)

When a Belarusian journalist photographs a protest, when a Sudanese civil-society documenter records an atrocity, when a Hong Kong activist captures arbitrary detention — the evidence is only as defensible as its **cryptographic chain of custody**. The dominant content-provenance standard, **C2PA (Content Credentials)**, has a published timestamp-substitution flaw (arXiv 2604.24890) and signed media can expire. The arXiv authors conclude: *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."*

The leading court-facing field-capture app, **eyeWitness to Atrocities** (International Bar Association), relies on a single corporate notary (LexisNexis) with no public anchor. The most-deployed open-source field tool, **Tella** (Horizontal), ships no integrity layer. **ProofMode** anchors photos to Bitcoin via OpenTimestamps but offers no structured evidence-bundle schema that a prosecutor or regulator can consume.

**Attestly** is an open public-anchor integrity layer that field-capture tools can adopt: an append-only signed ledger, Merkle-rooted Signed Tree Heads published to a public transparency log, and a prosecutor/regulator-runnable verifier. Decisions, photos, locations stay private to the capturing organisation; only short cryptographic commitments touch public infrastructure — the same architecture as **Certificate Transparency** for TLS authorities, applied to field evidence. **Apache-2.0** in perpetuity, embeddable, non-extractive.

Funding scope: a 12-week build for **three must-ship outputs** — a working Tella integration adapter, a drag-bundle-and-verify browser WASM verifier with three core localisations, and a frozen v0.1 specification — plus **stretch outputs** in standards engagement and partnership outreach.

---

## Current traction

The project is not vaporware; the core technology is shipped and runnable today.

- **Public repository** at github.com/attestly/attestly with **Apache-2.0** Rust workspace (3 crates, ~1,500 LoC). Clippy and rustfmt clean. CI matrix on Linux + macOS + Windows.
- **20 tests passing** — 18 unit, 2 end-to-end integration covering identity round-trips, append-only enforcement, Merkle proof correctness, signature verification, full tamper-detection pipeline.
- **End-to-end demo** reproducible by any reviewer in ~5 minutes (`cargo build --release && bash examples/demo.sh`). A 27-second screencast (`attestly-demo.gif`, 1.06 MB) shows initialise → 50 appended decisions → published signed checkpoint → exported evidence bundle → verification PASSES → operator tampers → re-verification FAILS with the exact hash mismatch reported.
- **Project website** at attestly.org live, with the demo embedded and the architecture documented.
- **Parallel grant submissions in preparation** to Sovereign Tech Fund and NLnet NGI Zero Commons, each scoped to disjoint deliverables (see *Non-overlap* below).
- **Initial ecosystem outreach planned for M3**: drafted briefs to Tella maintainers, International Bar Association eyeWitness team, and Witness/Guardian Project ProofMode contributors. We do **not** have signed letters of interest from these organisations yet; the OTF grant period is partly intended to fund the structured outreach that produces such evidence for future applications.

We are deliberately not over-claiming on ecosystem pull. The proposal is for an infrastructure layer, and adoption signals will be a primary M3 deliverable rather than a precondition.

---

## The Problem

### Field-evidence integrity is the unsolved layer in tools at-risk communities use

For journalists, human-rights documenters, and civil-society investigators in authoritarian or violently-policed environments, captured evidence must survive three adversaries to be useful:

1. **Confiscation and device seizure** — addressed by encrypted-at-rest mobile capture (Tella, ProofMode, Signal).
2. **Network interception during upload** — addressed by Tor/Signal-style anonymising transport.
3. **Disputed authenticity at the point of legal use** — partially addressed by C2PA and OpenTimestamps, but with documented gaps for high-stakes evidence.

The third layer is what determines whether captured evidence carries weight in an ICC case, a national human-rights tribunal, a press defamation suit, or a regulatory enforcement action.

### C2PA has a documented evidentiary weakness

C2PA is being adopted at scale — Adobe, Microsoft, BBC, AP, Reuters, NYT, and France Télévisions are in production with it; ProofMode is C2PA 2.3 conformant as of May 2026. But academic researchers in **arXiv 2604.24890** documented a timestamp-substitution attack class, and Arizona Secretary of State's January 2025 government-pilot images already fail validation a year later. The paper's literal conclusion: *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."*

This does not make C2PA useless — for newsroom-grade content provenance it is the right tool. It does mean that **for legal-evidence use cases** there is a layer C2PA does not currently address, and that layer requires an append-only public log against which a single piece of media can be verified years later without operator cooperation.

### The court-facing incumbent is closed-source and single-notary

**eyeWitness to Atrocities** (International Bar Association, since 2015, 85,000+ submissions, work on Ukraine and ICC casework) is the standard for legal-evidence field capture. Its chain-of-custody uses SHA-256 hashing into a private LexisNexis vault — a trust model that says "trust the IBA and LexisNexis" rather than "verify cryptographically." This is reasonable for the IBA's threat model; it is not necessarily the right long-term basis for a broader civil-society evidence commons, especially as more grass-roots and resistance journalism networks build their own tooling without IBA-scale notary partnerships.

### The open-source field-capture leader has no integrity layer

**Tella** (Horizontal, MIT, Android + iOS) is the most-deployed open-source secure-capture app for journalists and human-rights documenters. It handles encrypted storage, secure upload to org-controlled Uwazi/ODK/Nextcloud backends, and metadata capture. Its README does not document any hashing, signing, or public-anchor integrity scheme. A Tella-captured bundle, presented in court three years from now, depends on the trustworthiness of the receiving server's logs.

---

## The Solution: Attestly as a public-anchor backend

Attestly is a small framework-agnostic Rust library with three composable primitives that field-capture tools can adopt:

1. **An append-only signed event ledger.** Each captured-evidence record is canonicalised, hashed (SHA-256), signed (Ed25519 with the capture-tool's instance key), and appended to a hash-chained local ledger. Database triggers enforce append-only at the SQL layer.

2. **Publicly-published Signed Tree Heads.** Periodically, the ledger publishes a Merkle root over event hashes plus an Ed25519 signature by the organisation's `did:web` key. Only the commitments leave the operator's environment — never the captured photos, locations, or metadata.

3. **A regulator/prosecutor-runnable verifier.** A standalone CLI and a 200 kB browser WASM artefact. A prosecutor receives a single evidence bundle, drops it into the browser tool, and gets a mathematical verdict — independent of the capturing organisation's claims, independent of any vendor.

The architectural pattern is **Certificate Transparency** (RFC 9162) for TLS authorities and **Sigstore Rekor** for software supply chains, applied to field evidence. The underlying primitives are conventional and prior-art-supported (Merkle 1979; Haber & Stornetta 1991).

### Why Tella is the first integration target

Three concrete reasons beyond ecosystem prominence:

- **Technical accessibility**: Tella is open-source, with a documented Uwazi/ODK upload pipeline a third party can extend without requiring upstream cooperation. The first Attestly adapter can ship as a parallel package any Tella deployment can opt into.
- **Deployment footprint**: Tella is in use by Horizontal and partner organisations across multiple OTF-priority regions. An integration that lands cleanly with Tella reaches the OTF audience directly.
- **Civil-society resonance**: Tella's user base — journalists, human-rights documenters, civil-society investigators — is exactly the population whose evidence ends up in tribunals and court cases. The integration is on-thesis for OTF without requiring re-targeting.

ProofMode is intentionally not the first integration target: it already ships OpenTimestamps and C2PA 2.3 conformance, and Attestly's value-add for that ecosystem would be a structured-bundle schema rather than an anchoring primitive — useful, but not the highest-impact first build.

### Composition, not competition

Attestly does not duplicate ProofMode's OpenTimestamps anchoring, nor C2PA's content-credentials format. Where ProofMode is "this single image is timestamped" and C2PA is "this asset has provenance metadata," Attestly is "this entire evidence corpus has a structured verifier protocol with an open schema, with OpenTimestamps, Sigstore Rekor, or Google Tessera as supported anchor backends." ProofMode + Attestly compose. C2PA + Attestly compose. Tella + Attestly compose.

### The EU regulatory tailwind

**eIDAS 2 Regulation (EU) 2024/1183, Article 45l** recognises Qualified Electronic Ledgers with legal presumption of authenticity, with full implementation in December 2026. This does not guarantee court acceptance of any specific implementation, but it does establish — for the first time in EU regulatory text — that cryptographic ledgers are a recognised evidentiary instrument. Attestly is designed to be **a candidate open reference implementation under that framework**, not the only one and not pre-approved by any specific court, but architecturally aligned with the direction the regulation is heading.

---

## Beneficiaries

The direct beneficiaries are organisations and individuals already capturing evidence in environments where authenticity will later be contested:

- Journalists in restricted-press environments (Belarus, Russia, Hungary, parts of the Balkans, Turkey, multiple Sub-Saharan and Southeast Asian states), currently using Tella, Signal, ProofMode, or unencrypted tools.
- Human-rights documenters working under authoritarian governments, currently using Tella + Uwazi pipelines deployed by HURIDOCS, Witness, AccessNow's local partners.
- ICC and national-tribunal prosecutors, currently relying on eyeWitness submissions or accepting non-cryptographically-verified material with reduced evidentiary weight.
- Legal-aid clinics and human-rights NGOs preparing strategic litigation against state or corporate actors.
- Local-language community journalism networks (e.g., Bellingcat-trained regional volunteers, OCCRP affiliates) currently lacking structured chain-of-custody primitives.

Secondary beneficiaries — EU regulators, academic AI-accountability research groups — are outside the OTF-funded scope and addressed by parallel applications.

---

## Activities and Deliverables (12 weeks)

The plan is structured as **core build plus validation activities**. M1 and M2 contain the **must-ship outputs** that anchor delivery. M3 contains the **stretch outputs** (standards submission depth, partnership outreach depth, additional localisations) that we believe are achievable but explicitly mark as secondary so the project is not at risk if external timelines slip.

### M1 (end of week 4) — Tella integration adapter [must-ship]

- **Rust adapter** (`attestly-tella`) wrapping Attestly's append-only ledger and Signed-Tree-Head publication into Tella's existing upload flow.
- **Server-side wedge** for Tella's Uwazi/ODK/Nextcloud upload pipelines: a sidecar service that consumes Tella's REST upload and produces an Attestly-signed evidence bundle.
- **Working demo** against Tella's reference Uwazi instance.
- **Documentation** for any Tella-deploying organisation to adopt the adapter without requiring upstream merge. (An upstream PR is best-effort, not a deliverable gate.)

### M2 (end of week 8) — Browser WASM verifier + production transparency log [must-ship]

- **`verify.attestly.org`** — a 200 kB WASM artefact loaded by vanilla HTML, no JS framework. Drag a Tella or generic evidence bundle in, get a verdict in the browser. No data leaves the browser.
- **Production transparency log**: migration from MVP file-based STHs to Google Tessera (`transparency-dev`) Postgres-backed personality at `logs.attestly.org`.
- **Three must-ship localisations**: PT, ES, AR (selected to cover OTF priority regions with the highest civil-society documenter overlap). Additional localisations (FR, RU, UK) are stretch outputs, contracted out of the localisation budget only if the core deliverables ship on time.

### M3 (end of weeks 9-12) — Specifications + outreach [partly must-ship, partly stretch]

- **Must-ship**: Spec v0.1 frozen — Decision Schema, Checkpoint Format, Verifier Protocol — published at `spec.attestly.org` under CC-BY-4.0.
- **Must-ship**: Independent cryptographic review by an OTF-recommended reviewer, report received, blocker findings resolved.
- **Stretch**: IETF SCITT working group submission as Internet-Draft. The deliverable is the draft submission with IETF acknowledgement; WG adoption is downstream and out of grant scope.
- **Stretch**: International Bar Association partnership conversation initiated and documented. Deliverable is the written brief sent to the IBA eyeWitness team plus IBA's acknowledged response, not a signed partnership.

The must-ship outputs (Tella adapter, browser verifier, transparency log, specification, security review) form the project's evidence of execution and constitute the bulk of the engineering work. The stretch outputs (additional localisations, IETF submission depth, IBA depth) are validation activities whose external timelines we control imperfectly.

Each milestone unblocks 33% of the grant disbursement.

---

## Timeline and capacity

12 weeks from grant acceptance. Lead developer at 60% allocation; remaining 40% covers Conforme (the applicant's existing commercial product, providing the lead developer's primary income, decoupling Attestly's continued maintenance from grant cycles) and parallel grant administration. Buffer of ~10% on M3 to absorb security-review and outreach iteration.

The core engineering surface (Tella adapter, WASM verifier, Tessera migration, three localisations, spec freeze) is sized for one engineer at 60% over 12 weeks. The stretch activities (IETF submission, IBA outreach, FR/RU/UK localisations) are explicitly marked as such; reviewers should treat them as evidence of validation discipline rather than as commitments at risk.

---

## Budget (US $120,000)

The grant size reflects three considerations: (1) the engineering surface (~9 person-weeks of core build at €5,000/week-equivalent), (2) an independent cryptographic review at the rate OTF-recommended reviewers typically charge, and (3) localisation by professional translators rather than community volunteers (a deliberate choice — civil-society interface translations done by unpaid volunteers are a known reproducibility-and-quality risk). At a smaller grant size (~US $60-80k) we would cut the security review and localisation, which we believe are the items most likely to determine whether the artefact is actually adopted by civil-society organisations.

| Line item | Amount |
|---|---|
| Core engineering — Tella adapter, WASM verifier, Tessera log integration (12 weeks @ $6,000/week-equivalent, ~80% allocation) | $72,000 |
| Specifications work + standards engagement | $8,000 |
| Localisation — PT, ES, AR core (professional translator pool); FR, RU, UK stretch | $9,000 |
| Independent cryptographic review (OTF-recommended reviewer) | $12,000 |
| Production infrastructure — Tessera log hosting + Cloudflare Pages + CDN | $4,000 |
| IBA + Tella + civil-society partnership outreach + coordination | $5,000 |
| Travel — IETF or RightsCon equivalent civil-society conference for spec presentation | $4,000 |
| Project administration | $6,000 |
| **Total** | **$120,000** |

All amounts USD-denominated.

---

## Team

**Lead developer**: Curtis (Portugal-based, sole technical lead). 15+ years software engineering, multiple shipped products including **Conforme** (EU short-term-rental compliance SaaS, in production with paying customers across Portugal). Demonstrated end-to-end solo delivery on regulated-industry software.

**Co-maintainer**: A second contributor will be invited by M2 as a governance and continuity measure. To be transparent: no honorarium for this co-maintainer is budgeted in the line items above — the model is ecosystem-volunteer invitation, targeting an EU-resident contributor with prior Sigstore, `transparency-dev`, or Tella ecosystem experience. If a paid co-maintainer slot is needed for OTF's governance bar, we would reallocate from the indirect line; that decision will be made jointly with OTF programme staff if the application advances.

**Partnership advisors** (informal, pending outreach): International Bar Association eyeWitness team, Horizontal's Tella maintainers, Witness / Guardian Project ProofMode contributors.

---

## Why OTF

OTF funds open, reusable digital infrastructure for civil society and at-risk users, with a deliberate emphasis on tools that strengthen the underlying trust architecture of the open internet rather than vendor-extractive SaaS.

Attestly is the kind of artefact OTF historically funds: a small, framework-agnostic, permissively-licensed open library that the underlying capture-tool community can adopt, that prosecutors can run independently, and that does not extract a commercial dependency from the civil-society organisations that need it most. The pattern follows the logic OTF has supported for messaging primitives (Signal Protocol), circumvention infrastructure (Tor), and metadata-minimisation tools (CryptPad).

Four differentiation axes:

1. Open-source under Apache-2.0 in perpetuity — no commercial dependency.
2. Public-anchor rather than single-notary — distributes trust beyond any single vendor or government.
3. Framework-agnostic — Tella, ProofMode, eyeWitness, custom org tools can all adopt.
4. Aligned to eIDAS 2 Art 45l as a candidate reference implementation, rather than relying on private notary good behaviour.

---

## Open-source status

| Artefact | Licence | Repository |
|---|---|---|
| Rust core (`attestly-core`) | Apache-2.0 | `github.com/attestly/attestly` |
| CLI (`attestly-cli`) | Apache-2.0 | same |
| Verifier (`attestly-verifier`) | Apache-2.0 | same |
| Tella adapter (`attestly-tella`) | Apache-2.0 | same (post-M1) |
| Browser WASM verifier | Apache-2.0 | same (post-M2) |
| Specifications | CC-BY-4.0 | `github.com/attestly/spec` |
| Hosted commercial dashboard (post-grant, separate codebase) | proprietary | separate repository, never inside grant deliverables |

Governance: BDFL pattern documented in `GOVERNANCE.md` from day 1. Multi-maintainer onboarding by M2 as a continuity measure.

---

## Risks and Mitigations

1. **Tella maintainers do not adopt the adapter upstream.** Mitigation: the adapter ships as a parallel package any Tella deployment can opt into without upstream merge. Upstream PR is best-effort.

2. **IBA partnership conversation does not produce a commitment within 12 weeks.** Mitigation: the stretch deliverable is the *brief and acknowledged response*, not a signed partnership. The Tella integration and WASM verifier do not depend on IBA cooperation.

3. **IETF SCITT submission rejected or stalls.** Mitigation: the stretch deliverable is the submission itself plus IETF acknowledgement, not WG adoption.

4. **eIDAS 2 Art 45l implementation regulations slip beyond December 2026.** Mitigation: the project's technical value is independent of the eIDAS 2 timeline. Slippage weakens one positioning argument but not the deliverables.

5. **Single-applicant key-person risk.** Mitigation: multi-maintainer onboarding by M2; project artefacts are open-source from day 1 so continuity is not vendor-locked.

6. **Adoption pull weaker than projected.** Mitigation: the M3 outputs include the partnership-outreach work that produces concrete adoption signals for follow-on grant cycles; we explicitly do not pre-commit adoption in this concept note.

---

## Non-overlap with parallel applications

The applicant has parallel grant applications in preparation to:

- **Sovereign Tech Fund** — Attestly as EU AI Act Article 12 base infrastructure (the underlying library and EU-regulator-facing tooling).
- **EIC Accelerator Step 1 → ANI Voucher Deep Tech (Portugal)** — a commercial dashboard layer that sits *on top of* the open library, never duplicating it.
- **NLnet NGI Zero Commons** (October 2026 cycle) — Phase 3 federated multi-org logs and EUDI Wallet binding.

This OTF concept note is scoped to **civil-society and journalism evidence-integrity deliverables that are not in the STF or NLnet scope**: the Tella integration adapter, browser WASM verifier with civil-society localisation, IETF SCITT spec submission, and IBA partnership outreach. These deliverables would not be funded by the other applications; the boundaries are documented in `proposals/README.md`.

---

## References

- C2PA timestamp-substitution and expiry: arXiv [2604.24890](https://arxiv.org/abs/2604.24890)
- eIDAS 2: Regulation (EU) [2024/1183](https://eur-lex.europa.eu/eli/reg/2024/1183), Article 45l
- Certificate Transparency: [RFC 9162](https://datatracker.ietf.org/doc/rfc9162/)
- Sigstore Rekor: github.com/sigstore/rekor
- transparency-dev Tessera: github.com/transparency-dev/tessera
- Tella (Horizontal): github.com/Horizontal-org/Tella-Android
- ProofMode (Guardian Project): github.com/guardianproject/proofmode-android
- eyeWitness to Atrocities (International Bar Association): https://www.eyewitness.global
- IETF SCITT WG: https://datatracker.ietf.org/wg/scitt/about/

---

*Concept note prepared 2026-05-25, v2 after external review. Contact: hello@attestly.org · curts152@gmail.com · Project: https://attestly.org · Repository: github.com/attestly/attestly*
