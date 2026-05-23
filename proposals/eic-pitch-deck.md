# EIC Accelerator Step 1 — Pitch Deck

> 10 slides, content-only. Paste each slide into Google Slides, Keynote,
> or PowerPoint. Visual design: dark navy / charcoal slides, single
> accent colour (recommend a regulatory blue: `#2C5282`), white
> typography (Inter or DM Sans for headers, IBM Plex Mono for code/data).
>
> **Time to lay out**: 30-45 min in Google Slides if you keep design
> minimal. Use plenty of whitespace.

---

## Slide 1 — Cover

**ATTESTLY**

*Open verification infrastructure for EU AI Act Article 12 evidence*

Curtis Smith · curts152@gmail.com · github.com/attestly

EIC Accelerator Step 1 — Short Application
May 2026

---

## Slide 2 — The problem in one chart

**From 2 August 2026, every high-risk AI system in the EU must keep tamper-evident logs.**

A two-column visual:

| Column A — **What the law requires** | Column B — **What the market has** |
|---|---|
| Automatic logging over the system's lifecycle | Operator-controlled audit trails |
| Independent verifiability by regulators | Operator self-attestation only |
| 6-month minimum retention | Inconsistent storage |
| Penalties: €15M or 3% global turnover | No standard tooling |

**Speaker note**: This is a regulator-side problem, but it lands on the operator's compliance burden. ENISA estimates €0.5–10M per high-risk system for compliance.

---

## Slide 3 — The gap nobody owns

A simple 2×2 matrix:

```
                 │  Agentic AI    │  Non-agentic AI
─────────────────┼────────────────┼─────────────────
 In-process       │  Microsoft     │      ?
 verification     │  Agent         │
                  │  Governance    │
                  │  Toolkit       │
─────────────────┼────────────────┼─────────────────
 Regulator-       │      ?         │      ?
 runnable         │                │   ← THE GAP
 verification     │                │
```

**Speaker note**: Microsoft AGT covers one quadrant. Three are empty. Most high-risk AI under EU AI Act Annex III (credit scoring, biometric, medical diagnostic, employment screening) is **non-agentic** — falling outside AGT entirely. No open, regulator-runnable verification exists. Attestly fills the bottom-right.

---

## Slide 4 — What Attestly does

Three primitives, stitched into one drop-in library:

1. **Append-only signed event ledger** — every AI decision recorded with a cryptographic signature and a hash-chained reference to the previous decision.

2. **Public Signed Tree Heads** — cryptographic commitments (Merkle root + Ed25519 signature) published to a public transparency log. Only commitments are public. Decision payloads stay operator-controlled. GDPR-compatible by construction.

3. **Standalone CLI/WASM verifier** — a regulator runs it locally with no operator cooperation. Detects tampering mathematically.

**Architectural precedent**: Certificate Transparency for the web's PKI. Sigstore Rekor for software supply chains. Now AI decisions.

---

## Slide 5 — Working demo

**`bash examples/demo.sh`**

> 27-second screencast embedded here. The GIF shows:
> Init → 50 decisions appended → checkpoint published → evidence bundle exported → verify PASS → operator tampers → re-export → verify FAIL with the exact hash mismatch.

20 automated tests passing (18 unit + 2 e2e). Apache-2.0 (code) + CC-BY-4.0 (specifications). github.com/attestly/attestly.

**Speaker note**: TRL 5 at submission. Path to TRL 8 by month 24.

---

## Slide 6 — Market

**~20,000–50,000 EU operators** of Annex III high-risk AI systems by 2027. Growing 30% YoY.

| Segment | Operators | Avg WTP/year | Annual market |
|---|---|---|---|
| SME tier (€99–499/mo) | ~30,000 | €1,200–6,000 | €36M–180M |
| Enterprise tier (custom) | ~3,000 | €25,000–100,000 | €75M–300M |
| **Total EU TAM** | | | **€100M–500M** |

**24-month SOM**: €400K ARR (50–80 SME customers + 1–3 enterprise contracts) → €2.5M ARR trajectory by month 36.

---

## Slide 7 — Why us, why now

| Why us | Why now |
|---|---|
| Working PoC + 20 tests + screencast already shipped | Article 12 enforcement: **74 days away** |
| Track record: Conforme (EU compliance SaaS, paying customers in PT + ES) | Standards-body vacuum: CNCF, W3C have no AI-decision-event profile yet |
| Rust + Python + TypeScript polyglot; 15+ years shipping | Big-Tech vendors (Microsoft AGT) cover only agentic; rest of high-risk AI is uncovered |
| Apache-2.0 + CC-BY-4.0; perpetually open commons + paid SaaS layer | No EU sovereign alternative to US-hyperscaler verification stacks |

---

## Slide 8 — Business model

**Open-core**, with hosted commercial layer:

- **Free tier** (Apache-2.0 library + CLI + WASM verifier) — regulatory backstop, anchors the standard.
- **`attestly.cloud` SaaS** — hosted transparency log, multi-system dashboard, anomaly detection over signed log streams, automated regulator-pack export. €99–499/month SME; €5,000–25,000/year enterprise.
- **Professional services** — implementation consulting, custom integrations. Bounded; not the scale bet.

**Revenue trajectory** (mid case):
- Month 12: €30K ARR (PT + ES early pilots)
- Month 24: **€400K ARR** (50–80 SME + 1–3 enterprise)
- Month 36: €2.5M ARR trajectory (EU expansion + standards-body adoption flywheel)

---

## Slide 9 — Team + funding ask

**Team today**: Curtis Smith (founder, technical/product lead, Portugal). Track record: Conforme (production SaaS, paying customers PT + ES), 15+ years shipping regulated-industry software.

**Team by Month 12**: + co-maintainer (EU-resident, prior `spruceid/ssi` or Sigstore experience), + commercial lead.

**Advisors** (in flight outreach this week): Fraunhofer IAIS, INESC TEC HASLab, AlgorithmWatch, EU AI Act consultancy.

**Concurrent non-dilutive funding** (deliberately non-overlapping):

| Funder | Amount | Scope |
|---|---|---|
| Sovereign Tech Fund (in flight) | €100K | Open commons artefacts |
| **EIC Accelerator Step 2** (this ask) | **€1.8M grant + €1.2M optional equity** | Commercial scale-up |
| ANI Voucher Deep Tech (unlocked by Step 1 GO) | €60K + €10K | National PT match |
| NLnet NGI0 Commons (deferred Oct cycle) | €50K | Phase 3 scope |

---

## Slide 10 — Strategic interest to the EU

This is **EU regulatory trust architecture**, not a SaaS product.

- **AI Act enforcement infrastructure** — without open verification, the regulation goes from binding to advisory. Attestly is the missing protocol layer.
- **Digital sovereignty** — the verification layer is not Microsoft / Google / AWS controlled. Reference implementation is Portuguese; spec lives in CNCF or W3C; no US-hyperscaler in the trust chain.
- **EU competitive position in AI accountability** — first credible open standard published by an EU SME, before US equivalent traction. Standards-body capture is asymmetric.
- **Cross-border enforcement uniformity** — a regulator in Lisbon, Madrid, Berlin, or Paris uses the same `attestly verify --bundle` command on the same bundle format from the same standardised wire schema.

---

## Speaker notes — questions to anticipate

**"Why not just use Microsoft AGT?"**
AGT covers only agentic AI (a small minority of Annex III high-risk systems), runs in-process on the operator's infrastructure (regulator must trust operator), and explicitly publishes a non-stable Decision BOM schema (per AGT's own documentation, ruling out third-party tool ecosystems). Each is reasonable for a vendor-led project; none are acceptable for the *base infrastructure layer* on which a regulator-citizen-operator trust triangle should rest.

**"Isn't this an open-source project rather than a startup?"**
Both. The open commons artefacts are funded by Sovereign Tech Fund and stay Apache-2.0 + CC-BY-4.0 in perpetuity. The commercial scale-up — hosted dashboard, multi-system management, enterprise compliance reporting — is built *on top of* the commons artefacts and is what this EIC application funds. Same model as Sentry, Tailscale, Caddy.

**"How do you avoid being commoditised?"**
Three layers of moat: (1) first-mover open standard + multi-implementer network effect, (2) Conforme integration as proof of production deployment, (3) hosted SaaS dashboard with switching costs (the open library is free, the multi-system management isn't).

**"What's the worst-case scenario?"**
EIC Step 1 returns NO. We continue on Sovereign Tech Fund + Conforme revenue runway. The open commons ships regardless. Re-apply EIC Step 1 in 3 months with revised framing. The downside is contained.

**"What's the team gap?"**
Single founder is real. Hiring plan: co-maintainer by Sovereign Tech Fund M2; commercial lead by Month 12 with EIC funding.

---

## Visual design pointers (for Google Slides layout)

- **Background**: dark navy (`#1A202C`) for cover and section dividers; white (`#FFFFFF`) for content slides.
- **Accent**: regulatory blue (`#2C5282`) for highlights, key data points, calls-to-action.
- **Typography**: Inter / DM Sans for headers; IBM Plex Sans for body; IBM Plex Mono for code samples and demo output.
- **Slide 5 (demo)**: embed `attestly-demo.gif` as the central visual. Caption: "27 seconds, real cryptographic tamper detection."
- **Slide 6 (market)**: simple bar chart, no 3D, no excess. ENISA logo for credibility if available.
- **Slide 3 (gap)**: the 2×2 is the most important visual in the deck. Make it big. The "?" in the bottom-right cell is the entire pitch.

**Don't**: stock photos, person silhouettes, generic startup imagery, gradient overlays. The visual register should be technical and institutional, not consumer SaaS.
