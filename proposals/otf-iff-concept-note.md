# OTF Internet Freedom Fund — Concept Note

**Project**: Attestly — Public-anchor cryptographic evidence integrity for journalism and human-rights field documentation
**Funder**: Open Technology Fund — Internet Freedom Fund
**Funder portal**: https://www.opentech.fund/funds/internet-freedom-fund/
**Submission target**: rolling, monthly review
**Funding ask**: **US $120,000** over 12 months
**Lead applicant**: Curtis (Portugal-based, sole technical lead)
**Project repository**: https://github.com/attestly/attestly (public, Apache-2.0)
**Project website**: https://attestly.org

---

## Tagline (≤ 200 chars)

A public-anchor cryptographic backend that makes captured journalism and human-rights evidence court-admissible — closing the documented timestamp-substitution gap in C2PA, today's only mainstream content-provenance standard.

---

## Project Summary (≤ 250 words)

When a Belarusian journalist photographs a protest, when a Sudanese civil-society documenter records an atrocity, when a Hong Kong activist captures arbitrary detention — the evidence is only as defensible as its **cryptographic chain of custody**. The dominant content-provenance standard, **C2PA (Content Credentials)**, has a published timestamp-substitution flaw (arXiv 2604.24890) and signed media expires; Arizona's January 2025 government-pilot images already fail validation. The arXiv authors conclude literally: *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."*

The leading court-admissible field-capture app today, **eyeWitness to Atrocities** (International Bar Association), asks the world to trust a single corporate notary (LexisNexis). The most-deployed open-source field tool, **Tella** (Horizontal), ships no integrity layer at all. The strongest open primitive in the ecosystem, **ProofMode**, anchors to Bitcoin via OpenTimestamps but emits no structured evidence-bundle schema a regulator or prosecutor can consume.

**Attestly** is the missing public-anchor backend: an append-only signed ledger, Merkle-rooted Signed Tree Heads published to a public transparency log, and a regulator/prosecutor-runnable verifier. Decisions, photos, locations stay private to the operator; only short cryptographic commitments touch public infrastructure. Same architecture as **Certificate Transparency** for TLS, applied to high-stakes field evidence. **Apache-2.0** in perpetuity; permissive, embeddable, non-extractive.

Funding scope: a 12-week build to ship a working **Tella integration adapter**, a **drag-bundle-and-verify browser WASM tool**, a **partnership conversation with the International Bar Association** about a future eyeWitness v2 architecture, and a **submitted specification** to the IETF SCITT working group.

---

## The Problem

### Field-evidence integrity is broken in the tools at-risk communities actually use

For journalists, human-rights documenters, and civil-society investigators working in authoritarian or violently-policed environments, **captured evidence must survive three adversaries** to be useful:

1. **Confiscation and device seizure** — solved by encrypted-at-rest mobile capture (Tella, ProofMode).
2. **Network interception during upload** — solved by Tor/Signal-style anonymising transport.
3. **Disputed authenticity at the point of legal use** — *unsolved* by the tools currently deployed.

The third problem is the one that determines whether captured evidence is **admissible** in an ICC case, a national human-rights tribunal, a press defamation suit, or a regulatory enforcement action. And the third problem has a documented technical failure.

### C2PA is being adopted at scale, and it is broken for legal evidence

**C2PA (Content Credentials)** is the de-facto content-provenance standard — Adobe, Microsoft, BBC, the AP, Reuters, NYT, and France Télévisions are in production with it. ProofMode (Guardian Project) is C2PA 2.3 conformant as of v3.0.3 RC3 (May 2026).

But academic researchers in **arXiv 2604.24890** documented a timestamp-substitution attack class on C2PA, alongside an expiry pathology: **C2PA-signed media expires**, and Arizona Secretary of State's January 2025 pilot images already fail validation a year later. The paper's literal conclusion: *"C2PA should not yet be relied upon for high-stakes uses such as legal evidence."*

This is not a minor gap. It is the core difference between *content-provenance for newsrooms* and *evidence integrity for courtrooms*. The latter requires an append-only public log against which a single piece of media can be verified years later, without operator cooperation, by a prosecutor or human-rights lawyer who has no relationship to the capture tool's vendor.

### The court-admissible incumbent is closed-source and single-notary

**eyeWitness to Atrocities** (International Bar Association, since 2015, 85,000+ submissions, Ukraine and ICC casework) is the gold-standard field-capture app for legal evidence. Its chain-of-custody architecture uses SHA-256 hashing into a **private LexisNexis vault**. The trust model is "trust the IBA and LexisNexis"; there is no public anchor, no third-party verifiability, no open spec. It is closed-source and partnership-only.

This is reasonable for the IBA's threat model. It is **not reasonable** as the long-term basis for a global evidence-integrity commons, especially as more civil-society organisations and resistance journalism networks build their own tooling.

### The open-source field-capture leader has no integrity layer

**Tella** (Horizontal, MIT, Android + iOS) is the most widely deployed open-source secure-capture app for journalists and human-rights documenters. It does encrypted storage, secure upload to org-controlled Uwazi/ODK/Nextcloud backends, and metadata capture. But its README documents **no hashing, no signing, and no public anchor scheme** for integrity. A Tella-captured bundle, presented in court three years from now, is as defensible as the org-controlled server's logs — which is to say, easily disputed.

---

## The Solution: Attestly as a Public-Anchor Backend

Attestly is a small framework-agnostic Rust library with three composable primitives that any field-capture tool can adopt:

1. **An append-only signed event ledger.** Each captured-evidence record (photo, video, metadata bundle) is canonicalised, hashed (SHA-256), signed (Ed25519 with the capture-tool's instance key), and appended to a hash-chained local ledger. Database triggers enforce append-only at the SQL layer.

2. **Publicly-published Signed Tree Heads.** Periodically (or per-batch), the ledger publishes a Merkle root over all event hashes plus an Ed25519 signature by the operator org's `did:web` key. **Only the commitments leave the operator's environment** — never the captured photos, locations, or metadata. The public log is meaningless-looking hex; the captured evidence stays private.

3. **A regulator/prosecutor-runnable verifier.** A standalone CLI and a 200 kB browser WASM artefact. A prosecutor receives a single evidence bundle from the journalist's org, drops it into the browser tool, and gets a mathematical verdict — **independent of the journalist's claims, independent of the org's server, independent of any vendor**. If the underlying record was altered after the public Signed Tree Head was published, the verifier detects it with the exact hash mismatch reported.

The architectural pattern is **Certificate Transparency** (RFC 9162) for TLS authorities and **Sigstore Rekor** for software supply chains, applied to field-capture evidence integrity. The pattern is conventional, prior-art-supported, and within the cryptographic primitives a court can reason about (Merkle 1979, Haber & Stornetta 1991).

### Why this is not duplicating ProofMode or OpenTimestamps

**ProofMode** (Guardian Project) already anchors captured photo hashes to Bitcoin via OpenTimestamps. It is excellent at what it does. Attestly does **not compete with ProofMode** for the anchoring layer — OpenTimestamps wins that fight (Bitcoin-anchored, $0, decade of proven use).

Attestly's contribution is **the structured evidence-bundle schema and verifier protocol** that any field-capture tool can adopt, with **OpenTimestamps as one supported anchor backend** alongside Sigstore Rekor and Google Tessera. Where ProofMode is "this single image is timestamped," Attestly is "this entire evidence corpus has a regulator-runnable verification protocol with an open schema, embeddable in any capture tool."

ProofMode and Attestly compose. Tella and Attestly compose. eyeWitness — closed-source — adopts Attestly's spec for its v2 public-anchor architecture if the IBA partnership lands.

### Why the EU regulatory environment makes this urgent now

**eIDAS 2 Regulation (EU) 2024/1183, Article 45l** recognises **Qualified Electronic Ledgers** with legal presumption of authenticity, full implementation **December 2026**. For the first time, EU regulatory framework names cryptographic ledgers as evidentiary infrastructure. This is the tailwind that turns Attestly from "another transparency log" into "the open reference implementation that an EU court will accept" — and that European civil-society and journalism organisations can stand on without relying on US-vendor good behaviour.

---

## Beneficiaries

The direct beneficiaries are organisations and individuals already capturing evidence in environments where authenticity will later be contested:

- **Journalists in restricted-press environments** (Belarus, Russia, Hungary, parts of the Balkans, Turkey, multiple Sub-Saharan and Southeast Asian states) — currently using Tella, Signal, ProofMode, or unencrypted tools.
- **Human-rights documenters working under authoritarian governments** — currently using Tella + Uwazi pipelines deployed by HURIDOCS, Witness, AccessNow's local partners.
- **ICC and national-tribunal prosecutors** — currently relying on eyeWitness submissions or accepting non-cryptographically-verified material with reduced evidentiary weight.
- **Legal-aid clinics and human-rights NGOs** preparing strategic litigation against state or corporate actors — currently improvising chain-of-custody documentation.
- **Local-language community journalism networks** (e.g., Bellingcat-trained regional volunteers, MapBiomas Alerta, OCCRP affiliates) — currently lacking court-admissible primitives in their stack.

Secondary beneficiaries: EU regulators, market-surveillance authorities, and academic research groups working on AI accountability and content authenticity — but these are not the OTF-funded scope. (Article 12 of the EU AI Act is the parallel application thesis to other funders; this OTF concept note focuses on internet-freedom and civil-society users.)

---

## Activities and Deliverables (12 weeks)

### M1 (end of week 4) — Tella integration adapter

- **Rust adapter** (`attestly-tella`) integrating Attestly's append-only ledger and Signed-Tree-Head publication into Tella's existing Android/iOS capture flow.
- Server-side wedge for Tella's Uwazi/ODK/Nextcloud upload pipelines: a sidecar service that consumes Tella's REST upload and produces an Attestly-signed evidence bundle.
- Working demo on Tella's reference Uwazi instance.
- Pull-request to Tella's main repository (Horizontal-org/Tella-Android) with the integration documented; merge or not is downstream, the artefact is what we deliver.

### M2 (end of week 8) — Browser WASM verifier + production transparency log

- **`verify.attestly.org`** — a 200 kB WASM artefact loaded by a vanilla HTML page (no JS framework dependency). Drag a Tella or generic evidence bundle in, get a verdict in the browser. No data leaves the browser.
- Migration from MVP file-based STHs to Google Tessera (`transparency-dev`) Postgres-backed personality. `logs.attestly.org` live, serving signed checkpoints and inclusion proofs over HTTPS.
- Localisations: PT, ES, FR, RU, UK, AR for the verifier UI (six languages chosen to cover OTF priority regions).

### M3 (end of week 12) — Specifications + partnership outreach + spec submission

- **Spec v0.1 frozen**: Decision Schema, Checkpoint Format, Verifier Protocol. Published at `spec.attestly.org` (CC-BY-4.0).
- **Submitted as an Internet-Draft to the IETF SCITT working group**, which standardises receipts for supply-chain transparency — Attestly's schema is a natural sister specification.
- **International Bar Association partnership conversation initiated and documented**: a written brief sent to the IBA eyeWitness team proposing Attestly as the public-anchor primitive for a future eyeWitness v2 architecture. The deliverable is the brief + IBA's acknowledged response, not a signed partnership.
- Independent code review by an OTF-recommended cryptographic reviewer (or equivalent), report published.

Each milestone unblocks 33% of the grant disbursement.

---

## Timeline

12 weeks from grant acceptance. Lead developer at 60% allocation; remaining 40% covers Conforme (the applicant's existing commercial product) and parallel grant submissions to Sovereign Tech Fund, NLnet NGI0, EIC Step 1 / ANI Voucher. Buffer of ~10% on M3 to absorb security-review and IBA-outreach iterations.

---

## Budget (US $120,000)

| Line item | Amount |
|---|---|
| Core engineering — Tella adapter, WASM verifier, Tessera log integration (12 weeks @ $6,000/week, 80%) | $72,000 |
| Specifications + IETF SCITT submission + standards engagement | $10,000 |
| Localisation — PT, ES, FR, RU, UK, AR (6 languages, professional translator pool) | $9,000 |
| Independent cryptographic review (OTF-recommended reviewer) | $12,000 |
| Production infrastructure — Tessera log hosting + Cloudflare Pages + CDN | $4,000 |
| IBA partnership outreach + civil-society organisation pilot coordination | $5,000 |
| Travel — IETF + RightsCon 2027 or equivalent civil-society conference for spec presentation | $4,000 |
| Indirect & administration (3.3%) | $4,000 |
| **Total** | **$120,000** |

Project cost exceeds the typical Internet Freedom Fund single-applicant minimum. All amounts USD-denominated.

---

## Team

**Lead developer**: Curtis (Portugal-based, sole technical lead). 15+ years software engineering, multiple shipped products including **Conforme** (EU short-term-rental compliance SaaS, in production with paying customers across Portugal and Spain) and prior Solana/blockchain infrastructure work. Demonstrated end-to-end solo delivery on regulated-industry software. Has internalised the EU AI Act and adjacent regulations through prior compliance work.

**Co-maintainer**: One additional contributor will be invited by M2 to satisfy multi-maintainer governance (no single-applicant lock-in on the spec or namespace). Target: an EU-resident contributor with prior Sigstore, `transparency-dev`, or Tella ecosystem experience.

**Partnership advisors** (informal, pending outreach): International Bar Association eyeWitness team; Horizontal's Tella maintainers; Witness / Guardian Project ProofMode contributors.

---

## Why OTF

OTF funds **digital base infrastructure for at-risk users and civil society**, with a deliberate emphasis on tools that strengthen the underlying trust architecture of the open internet — not vendor-extractive SaaS, not surveillance platforms, not products that only work for resourced operators in democratic states.

Attestly is the kind of artefact OTF historically funds: a small, framework-agnostic, permissively-licensed open library that **the underlying capture-tool community can adopt**, that **prosecutors can run independently**, and that **does not extract a commercial dependency** from the civil-society organisations that need it most. The pattern is the same logic OTF supports for messaging primitives (Signal Protocol), circumvention infrastructure (Tor), and metadata-minimisation tools (CryptPad).

The four axes Attestly differentiates on:

1. **Open-source under Apache-2.0 in perpetuity** — no commercial dependency.
2. **Public-anchor, not single-notary** — distributes trust beyond any single vendor or government.
3. **Framework-agnostic** — Tella, ProofMode, eyeWitness, custom org tools can all adopt.
4. **EU-regulator-grade primitive** — anchored to eIDAS 2 Art 45l rather than a private notary's good behaviour.

---

## Open-source status

| Artefact | Licence | Repository |
|---|---|---|
| Rust core (`attestly-core`) | Apache-2.0 | `github.com/attestly/attestly` |
| CLI (`attestly-cli`) | Apache-2.0 | same |
| Verifier (`attestly-verifier`) | Apache-2.0 | same |
| Tella adapter (`attestly-tella`) | Apache-2.0 | same |
| Browser WASM verifier | Apache-2.0 | same |
| Specifications (Decision Schema, Checkpoint Format, Verifier Protocol) | CC-BY-4.0 | `github.com/attestly/spec` |
| Hosted commercial dashboard (post-grant, separate codebase) | proprietary | separate repository, never inside grant deliverables |

Governance: BDFL pattern documented in `GOVERNANCE.md` from day 1. Multi-maintainer onboarding by M2.

---

## Risks and Mitigations

1. **IBA partnership conversation does not land within 12 weeks.** Mitigation: M3 deliverable is the *brief and acknowledged response*, not a signed partnership. The Tella integration and WASM verifier do not depend on IBA cooperation.

2. **Tella maintainers reject the upstream pull-request.** Mitigation: the adapter ships as a parallel package (`attestly-tella`) that any Tella deployment can opt into without requiring upstream merge. The pull-request is best-effort, not a deliverable gate.

3. **IETF SCITT submission rejected or stalls.** Mitigation: the submission itself, with IETF acknowledgement, is the deliverable. WG adoption is downstream and not in grant scope.

4. **eIDAS 2 Art 45l implementation regulations slip beyond December 2026.** Mitigation: the project's architectural value is independent of the eIDAS 2 timeline. Slippage would weaken one positioning argument but not affect the technical deliverables.

5. **Single-applicant key-person risk.** Mitigation: governance + multi-maintainer onboarding by M2; project artefacts are open-source from day 1 so continuity is not vendor-locked.

---

## Non-overlap with parallel applications

The applicant has parallel grant applications to:
- **Sovereign Tech Fund** (Attestly as EU AI Act Article 12 base infrastructure; scope is the underlying library and EU-regulator-facing tooling).
- **EIC Accelerator Step 1 → ANI Voucher Deep Tech (Portugal)** (commercial dashboard layer that sits *on top of* the open library, never duplicating it).
- **NLnet NGI Zero Commons** (October 2026 cycle; Phase 3 federated multi-org logs and EUDI Wallet binding).

This OTF concept note is scoped to **civil-society and journalism evidence-integrity deliverables that are not in the STF or NLnet scope**: specifically the Tella integration adapter, browser WASM verifier with civil-society localisation, IBA partnership outreach, and IETF SCITT spec submission. These deliverables would not be funded by the other applications; the boundaries are documented in `proposals/README.md`.

---

## References

- **C2PA timestamp-substitution and expiry**: arXiv [2604.24890](https://arxiv.org/abs/2604.24890)
- **eIDAS 2**: Regulation (EU) [2024/1183](https://eur-lex.europa.eu/eli/reg/2024/1183), Article 45l (Qualified Electronic Ledgers)
- **Certificate Transparency**: [RFC 9162](https://datatracker.ietf.org/doc/rfc9162/)
- **Sigstore Rekor**: github.com/sigstore/rekor
- **transparency-dev Tessera**: github.com/transparency-dev/tessera
- **Tella** (Horizontal): github.com/Horizontal-org/Tella-Android
- **ProofMode** (Guardian Project): github.com/guardianproject/proofmode-android
- **eyeWitness to Atrocities** (International Bar Association): https://www.eyewitness.global
- **IETF SCITT**: https://datatracker.ietf.org/wg/scitt/about/

---

*Concept note prepared 2026-05-25. Contact: hello@attestly.org · curts152@gmail.com · Project: https://attestly.org · Repository: github.com/attestly/attestly*
