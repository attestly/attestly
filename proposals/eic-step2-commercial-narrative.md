# EIC Step 2 — Commercial scaleup narrative (parking memo)

**Status**: parking memo, not an active proposal. Develop into an EIC Accelerator Step 2 application **only if Step 1 returns GO**.

**When to use**: only for the commercial scaleup layer (the hosted dashboard service that sits *on top of* the Apache-2.0 library). **Do NOT use this framing in NLnet, STF, OTF, ISF, ESA, or Vale Inovação applications** — those funders evaluate on specific OSS-infrastructure deliverables; a "trust platform" narrative reads as commercial-overreach and damages those applications.

---

## The strategic insight (from external review 2026-05-26)

The Attestly primitive is positioned narrowly for the grant pipeline — EU AI Act Article 12 + eIDAS 2 Art 45l + civil-society evidence integrity. That positioning is correct for non-repayable funders and should not be diluted.

But the underlying market category is broader. Every regulated sector that depends on "a single party produces and stores evidence about itself" inherits the same integrity gap that Article 12 names — and generative AI makes the gap acute by making synthetic documents, fake audit trails, and manipulated evidence trivial to produce. The post-AI-fraud regulatory environment is creating a category that EU AI Act + eIDAS 2 are just two slices of.

The competitive landscape today (DocuSign, ID.me, Trulioo, Vanta, Drata, OneSpan, ChainIT, Autenti, Encompass) is **siloed and industry-specific**. No single company owns the broader narrative of *"trust infrastructure for the AI era."* That category is open.

---

## The commercial scaleup thesis

**Attestly the open library remains the upstream primitive — Apache-2.0 in perpetuity. The commercial scaleup is a hosted dashboard service that adopts the same primitive for cross-sector regulated trust workflows**, sold to organisations that need the integrity guarantees but lack the in-house engineering to operate the library + transparency log + verifier themselves.

The hosted layer is where "Stripe for trust" / "Cloudflare for verification" framing applies. The open library is where Sigstore, OpenSSL, curl framing applies.

The two layers compose:

| Layer | Licence | Market | Funder |
|---|---|---|---|
| **Library + verifier + spec** (`github.com/attestly/attestly`) | Apache-2.0 + CC-BY-4.0 in perpetuity | Open ecosystem; any organisation can self-host | NLnet, STF, OTF, ISF, ANI Voucher |
| **Hosted dashboard service** (post-grant, separate codebase) | Proprietary | EU-regulated SMEs in finance, legal, construction, government, property, AI, insurance | EIC Accelerator Step 2; ETCI; commercial revenue |

---

## Cross-sector use-cases the hosted layer can address

Mapping the reviewer's fragmentation observation to specific commercial slices Attestly's hosted layer can address (post-grant):

| Sector | Their integrity concern | Attestly hosted layer fit |
|---|---|---|
| **Finance** | KYC/AML audit trail tamper-evidence under EU AMLR (2024/1624) | Hosted ledger + verifier-as-a-service for compliance officers |
| **Legal** | Evidence chain-of-custody for cross-border litigation | Attestly verifier as an eIDAS 2 Qualified Electronic Ledger consumer |
| **Construction** | Site-inspection logs under EU Building Performance Directive | Field-evidence integrity (same primitive as civil-society) |
| **Government grants** | Procurement and fraud audit | Cryptographic audit log for grant disbursement workflows |
| **Property compliance** | Conforme already addresses this; could federate via Attestly | Multi-property-manager integrity attestation |
| **AI providers** | EU AI Act Article 12 (primary grant scope) | Hosted Article-12-evidence-pack-as-a-service |
| **Insurance** | Claims-evidence integrity post-AI-fraud | Claims-evidence cryptographic anchoring |

---

## Why this narrative is right for EIC Step 2 specifically

EIC Step 2 evaluates on three axes: **innovation, market potential, scale-up capacity**. The broader-category framing maps directly:

- **Innovation**: open public-anchor primitive is a category-creating piece of infrastructure (Sigstore Rekor pattern applied to non-supply-chain evidence).
- **Market potential**: cross-sector regulated-trust market is in the tens of €B by 2030 (combined DocuSign / Vanta / Drata / ID.me / Trulioo / OneSpan addressable market), with EU regulatory tailwind across every domain.
- **Scale-up capacity**: the open library is already shipped (v0.1 working, 20+ tests passing, 5 grant proposals in flight). Step 2 funds the commercial hosted layer on top of demonstrated open-source traction.

---

## What needs to be done before this narrative is usable

This is a parking memo, not a draft. The work required to turn it into an EIC Step 2 application:

1. **EIC Step 1 GO** (prerequisite — without it, Step 2 isn't available).
2. **ANI Voucher Deep Tech (€60k + €10k consultancy)** — mechanically triggered by Step 1 GO; funds the Lda incorporation and initial commercial-layer engineering.
3. **First commercial pilot** — Conforme is the natural first paying customer for the hosted layer (already integrated, already PT-regulated). Need at least one non-Conforme paying pilot to prove cross-sector applicability before Step 2 application.
4. **Two named EU enterprise letters of interest** — common Step 2 requirement; outreach to potential adopters in finance + insurance + construction can start once OTF/STF results are known.
5. **3-year financial projection + use-of-funds breakdown** — standard Step 2 documentation.

Timeline if Step 1 GO arrives in ~July 2026: Step 2 application achievable for Q1 2027 submission window.

---

## Companies to monitor

These are not competitors to the open library (Apache-2.0 OSS competes on adoption, not on revenue), but they ARE potential competitors to the hosted commercial layer:

- DocuSign (signatures + audit trails)
- ID.me (identity verification, US-leaning)
- Vanta, Drata, Secureframe (SOC 2 / GRC compliance trails)
- OneSpan (digital agreement integrity)
- Trulioo (KYC/identity verification)
- ChainIT (blockchain-anchored audit trails)
- Encompass Corporation (KYC/AML workflow)
- Autenti (EU electronic signature)

None of them currently address the EU AI Act Article 12 evidentiary integrity gap, and none operate at the public-anchor / open-spec layer. Attestly's commercial moat is the same Apache-2.0 + open spec that the grant funders fund — competitors building closed alternatives lose against an open primitive that EU regulators can independently verify.

---

## Reference: the reviewer's observation that triggered this memo

External review 2026-05-26 surfaced the "wider market than AI Act Article 12" observation. The recommendation was to re-position Attestly's website hero around "trust verification layer" / "trust infrastructure for the AI era." That recommendation was **rejected** for the open-library positioning (would damage the in-flight grant pipeline) but captured here for the commercial scaleup phase where the framing is appropriate.

See decision rationale in this commit's parent conversation. Three targeted moves applied 2026-05-26:

1. Added § 5 "Applicability" section to `landing/index.html` acknowledging broader applicability without changing the hero
2. Added Arc 4 to `mcgovern-ai-for-humanity-prep.md` capturing the broader-category framing for McGovern specifically (mission-aligned funder where it's appropriate)
3. This parking memo (the commercial scaleup narrative for Step 2)

---

*Parking memo prepared 2026-05-26. Develop only on Step 1 GO. Contact: hello@attestly.org · curts152@gmail.com*
