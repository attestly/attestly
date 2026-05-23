# EIC Accelerator Step 1 — Short Application — Attestly

> European Innovation Council Accelerator, Step 1 (short application).
> Submission target: next monthly cut-off after **Sat 31 May 2026**.
> Funder portal: EU Funding & Tenders Portal (https://ec.europa.eu/info/funding-tenders/opportunities/portal/).
> A GO from Step 1 unlocks the **ANI Voucher Deep Tech (€60k + €10k consultancy)** automatically, plus eligibility for Step 2 (€0.5–2.5M grant + €0.5–10M equity).

---

## Administrative

| Field | Value |
|---|---|
| Acronym | ATTESTLY |
| Project title | Open verification infrastructure for EU AI Act Article 12 evidence |
| Total estimated budget (Step 2 indicative) | €1,800,000 (grant) + €1,200,000 (equity, optional) |
| Project duration | 24 months |
| Coordinator | Curtis (Portugal, sole founder; Sociedade Unipessoal por Quotas incorporation in flight per applicant's pre-Step-2 plan) |
| Sector | Cybersecurity / Regulatory Technology / Deep-Tech AI |
| Strategic Technology priority | AI safety + accountability infrastructure (EIC priority cluster) |

---

## 1. Innovation

### 1.1 Value proposition

Attestly is **open, framework-agnostic regulatory-trust infrastructure for EU AI Act Article 12 evidence**. It records AI decision events in an append-only signed ledger, publishes cryptographic commitments — never decision payloads — to a public transparency log, and provides a regulator-runnable verifier that operates without trusting the AI operator. Any single decision made by any high-risk AI system can be exported as an evidence bundle and verified mathematically by a regulator, an affected citizen, or a court.

Article 12, enforceable from **2 August 2026**, requires every operator of a high-risk AI system per Annex III (credit scoring, hiring, biometric, employment, essential services, law enforcement, migration, justice) to maintain automatic logs of operation. The regulation is silent on whether those logs are tamper-evident. Today an operator can edit their own audit trail before a regulator inspects it; if challenged in a national court or before a market-surveillance authority, **the operator's audit trail is not admissible as integrity-verified evidence** — only as the operator's own assertion. Penalties for record-keeping failures reach **€15M or 3% of global turnover** (Article 99). Attestly closes the gap: every Article 12 log entry becomes **independently verifiable evidence** rather than operator-attested record.

### 1.2 Unique selling point

**The world's first open, framework-agnostic, regulator-runnable verification layer for AI Act Article 12 evidence.** Three concurrent properties make Attestly distinct:

1. **Framework-agnostic.** Microsoft Agent Governance Toolkit (April 2026) is the closest market reference but scopes only to agentic AI within its own framework stack. Attestly accepts CloudEvents from any source (Anthropic, OpenAI, custom ML pipelines, AGT itself), covering the broader population of high-risk AI which is *not* agentic (credit scoring, medical diagnostic, biometric — single-inference systems that AGT does not address).

2. **Regulator-independent verification.** Existing AI audit tooling (Langfuse, AgentOps, Helicone, AGT) is operator-side: the regulator sees what the operator chooses to show. Attestly publishes cryptographic commitments to a public transparency log. A regulator runs `attestly verify --bundle` locally with no operator cooperation and detects tampering mathematically. Same pattern as Certificate Transparency for the web's PKI.

3. **GDPR-compatible by construction.** Only hashes and signed checkpoints are public; decision payloads stay operator-controlled. The architectural separation — public commitments, private payloads — sidesteps every objection that "tamper-evident logging = privacy exposure". Industry precedent: Sigstore's Rekor uses the same pattern for software supply chains.

### 1.3 Technology and IP

**Core technology**: Ed25519 signing + SHA-256 Merkle trees + Signed Tree Heads, integrated with W3C DID Core 1.0 identity (`did:web`), CloudEvents v1.0 envelope format, and `transparency-dev`'s Tessera for production transparency-log hosting. All primitives are well-understood cryptographic building blocks; the **innovation lies in the system-level integration and the standardised wire format** that makes regulator-runnable verification practical at SME scale.

**Patent posture**: deliberately none. Attestly is published under Apache-2.0 (code) + CC-BY-4.0 (specifications) in perpetuity. The Decision Schema and Checkpoint Format are filed (M4) with either CNCF CloudEvents WG or W3C Community Group as open standards. The defensibility is in the *combination* — schema + verifier + transparency-log adapter + regulator UX — and in being the first credible open implementation to ship to the EU market in advance of Article 12 enforcement.

**Trade-secret IP** (proprietary, in the commercial layer separate from the open commons): the hosted multi-system management dashboard built atop the open core. Anomaly-detection heuristics over signed log streams, fleet-level compliance dashboards, automated regulator-evidence-pack generation across multiple AI systems — all on the SaaS side, never inside the grant-funded open libraries.

### 1.4 Technical Readiness Level (TRL)

- **At Step 1 submission**: TRL 5 — system validated in relevant environment. Working Rust implementation (3 crates, 20 tests passing, ~1500 lines), full end-to-end demo with cryptographic tamper detection working on real regulatory-style decision payloads. Demonstrated on a 50-event synthetic credit-scoring workload, plus a real integration target (Conforme's NRUA decisioning wizard) ready as the first pilot.
- **End of Step 2 (Month 24)**: TRL 8 — system complete and qualified. v1.0 shipped, security review by Radically Open Security passed, three commercial pilots in production (Conforme + two non-applicant operators), Decision Schema accepted by either CNCF or W3C as a contributed profile.

### 1.5 Differentiation from current solutions

| Solution | Scope | Verifiability | Multi-vendor | Open licence |
|---|---|---|---|---|
| **Microsoft AGT** | Agentic AI only | Operator-side, in-process | OpenAI-centric | MIT |
| **Langfuse, AgentOps, Helicone** | LLM observability | None (no integrity guarantees) | Multi-LLM | OSS/SaaS |
| **Lakera Guard, NeMo Guardrails** | Prompt-injection / safety guardrails | N/A — different problem | Multi-LLM | Mixed |
| **Vanta, Drata, Secureframe** | SOC2/ISO27001 attestation orchestration | Attestation-flow only; no crypto primitives | N/A | Commercial only |
| **craevidence.com** | EU CRA Annex I evidence (Cyber Resilience Act, different regulation) | Commercial only, closed source | N/A | Closed |
| **Attestly** | **All high-risk AI per AI Act Annex III** | **Regulator-runnable, third-party verifiable, no operator trust required** | **Framework-agnostic** | **Apache-2.0 + CC-BY-4.0** |

---

## 2. Market

### 2.1 Problem and need

The EU AI Act creates a **mandatory €0.5–3M-per-system compliance obligation** for every operator of an Annex III high-risk AI system from August 2026 onwards. ENISA's own preparatory analysis estimates per-system implementation costs at €0.5–10M depending on system complexity. The audit-log integrity sub-problem alone — what Attestly addresses — represents the single hardest unsolved component of compliance, because no off-the-shelf solution exists today for non-agentic high-risk systems.

The pain is **acute, dated, and unevenly distributed**. Aug 2 2026 is binding; there is no preparation period for evidence-integrity tooling that operators can rely on. Today, every EU bank making automated credit decisions, every HR platform doing CV screening, every healthcare provider running diagnostic AI is technically a few months from a regulatory regime they cannot fully satisfy with existing tooling. The **compliance burden falls hardest on SMEs**, who cannot afford to build proprietary audit-integrity systems and have no standardised alternative. A single drop-in open primitive — exactly what this project delivers — converts a €1-3M custom build into a zero-marginal-cost dependency.

### 2.2 Addressable market

- **Total Addressable Market (TAM)**: Every EU-located operator of an Annex III high-risk AI system. ENISA + industry estimates put this at **20,000–50,000 operators across the EU**, growing 30% YoY as more AI applications come into scope. At a conservative average compliance-tooling spend of €5,000/year per operator, EU TAM is **€100M–250M annual SaaS market** by 2028. Bottom-up: financial services (~2,000 EU institutions), HR platforms (~5,000), healthcare AI (~3,000), public sector (~3,000), recruitment (~10,000), credit bureaus + lending (~5,000).

- **Serviceable Addressable Market (SAM)**: Operators with 100+ employees and active compliance budget. Estimated **8,000–12,000 EU operators**. Average willingness-to-pay €1,200–6,000/year for SaaS at SME tier, €25,000–100,000/year for enterprise tier. SAM: **€30M–80M annual recurring revenue ceiling**.

- **Serviceable Obtainable Market (SOM) at 24-month horizon**: 0.5–1% market penetration realistic. **€150K–800K ARR by end of Month 24**, with strong growth trajectory if the open standard achieves traction inside CNCF or W3C (Month 18–24).

### 2.3 Customer segments

- **Primary**: EU mid-market and enterprise operators of Annex III high-risk AI systems (financial services, HR-tech, healthtech, identity verification). Procurement path: compliance officer + CISO, 2–6-month sales cycle, €5K–50K/year average contract value.
- **Secondary**: AI consultancies and Big-4 advisory practices selling Article-12 readiness services. Channel partners, not direct customers — 20–30% revenue share on referrals.
- **Tertiary**: EU regulators and national AI authorities adopting the open verifier as their inspection tool. Zero revenue, but standards-body and reference-implementation positioning is the strategic moat.

### 2.4 Business model

**Open-core, with hosted commercial layer:**

- **Free tier**: Apache-2.0 library + CLI + WASM verifier. Self-host. Every operator can deploy at zero licence cost. This is the regulatory backstop that anchors the standard.
- **SaaS tier — `attestly.cloud`**: Hosted transparency log (Tessera-backed), multi-system management dashboard, anomaly detection over signed log streams, automated regulator-export bundle generation, audit-ready compliance reports. **€99–499/month for SME tier (1–10 high-risk AI systems), €5,000–25,000/year for enterprise**. AGPL-3.0 for self-hosted distribution of the dashboard; proprietary for the hosted instance.
- **Professional services tier**: Implementation consulting, custom integrations, regulator-engagement support. €1,500/day. Bounded — not the scale-up bet.

Revenue projection (mid-case):
- End of Month 12: €30K ARR (early-adopter SME pilots, PT + ES focus).
- End of Month 24: **€400K ARR** (50–80 paying SME customers + 1–3 enterprise contracts).
- End of Month 36: **€2.5M ARR** trajectory (EU expansion, standards-body adoption flywheel).

### 2.5 Competition (already-shipped solutions)

| Competitor | Why we win | Why we might lose |
|---|---|---|
| Microsoft AGT | Vendor-neutral framing matters to non-Microsoft operators; we cover non-agentic AI which AGT does not | Microsoft brand is the default safe choice |
| Langfuse / AgentOps | Integrity guarantees vs observability only | They have established customers; switching cost |
| Vanta / Drata | Specific to AI Act Article 12 (vs generic SOC2 evidence flow) | They have sales muscle and existing relationships |
| In-house "we'll build it ourselves" by regulated industry | Standardised approach with multi-implementation network effect | Big banks may always prefer in-house for differentiation |
| Future entrants in 2026–2027 | First-mover + Apache-2.0 open standard + standards-body submission | Late entrants with deeper pockets may copy the open spec |

**The key defensive advantage**: open-source perpetual licence + standardised wire format makes the *protocol* the moat, not any proprietary technology. Operators building against the Decision Schema are switching-cost-neutral toward Attestly's commercial layer vs alternatives — but the dashboard SaaS gets first-mover network effects (one published standard, multiple operators' STHs verifiable by one regulator UI).

### 2.6 Go-to-market

- **Month 1–6 (post-EIC GO)**: Two named pilots — Conforme (already integrated by M2 of Sovereign Tech Fund deliverables) plus one non-applicant EU operator (target: Portuguese or Spanish bank's KYC/credit-decisioning team via warm introduction from Conforme's existing network).
- **Month 6–12**: Decision Schema accepted by CNCF CloudEvents WG or W3C CG as a contributed profile. This is the asymmetric event — once accepted, every other CloudEvents-aware system can interoperate. We become the reference implementation.
- **Month 12–18**: Channel partnerships with 2–3 EU compliance consultancies; advisor relationships with one academic institution (target: Fraunhofer SIT or INESC TEC) and one civil-society organisation (target: AlgorithmWatch).
- **Month 18–24**: First enterprise contracts (target: a tier-2 European bank or a EU-funded data-space initiative).

---

## 3. Implementation

### 3.1 Team

**Curtis** — sole founder + technical lead. Portugal-based; 15+ years software engineering. Demonstrated track record of regulated-industry SaaS delivery (Conforme — EU short-term rental compliance SaaS, currently in production with paying customers in PT + ES; multi-agent AI systems orchestration; blockchain / Solana development; Rust + Python + TypeScript polyglot). Internalised the EU AI Act through 6 months of Conforme's Layer 2 (NRUA registration wizard) build — a system that itself emits Article 12-style decisions and is the natural first pilot for Attestly.

**Co-maintainer and senior engineer** (to hire by Month 6): EU-resident, prior experience with one of `spruceid/ssi`, Sigstore, `transparency-dev`, or CloudEvents WG. Outreach targets identified; offers extended at Sovereign Tech Fund M2 milestone (parallel funded work in flight — see §4 below).

**Commercial / sales lead** (to hire by Month 12): EU-experienced sales lead with compliance-software background. Compensation: heavily variable comp tied to ARR milestones.

**Advisory board**: 3–5 advisors at €0 cost. Targets confirmed-in-conversation: one academic (EU AI Act-literate), one regulator-side (national AI authority observer), one civil-society (AlgorithmWatch / EDRi), one compliance-industry (Big-4 / Sia Partners), one investor (sector-relevant European VC).

### 3.2 Resources needed

**EIC Step 2 budget (indicative)**: €1,800,000 grant + €1,200,000 optional equity-component.

| Category | Amount | Notes |
|---|---|---|
| Personnel (3 FTEs avg over 24 months: founder + co-maintainer + commercial lead) | €1,000,000 | Sustained development + standards engagement + commercial ramp |
| External engineering contractors (SDK ports, web verifier polish) | €150,000 | Strategic outsource of non-core surface |
| Independent security reviews (annual + pre-v1.0 + pre-v2.0) | €60,000 | Critical for regulator credibility |
| Standards-body engagement (CNCF/W3C participation, IETF SCITT contributions, conference travel) | €40,000 | Defensive moat investment |
| Pilot subsidies (50% match toward first 3 enterprise pilots) | €120,000 | Customer-acquisition cost reduction |
| Hosting + infrastructure (Tessera transparency log, verifier CDN, monitoring) | €70,000 | M2 onwards, 24-month commitment |
| Legal + regulatory + compliance (DPA, security policies, EU entity establishment, IP) | €60,000 | Mandatory operational overhead |
| Marketing + community + maintainer compensation | €300,000 | Sustained community building |
| **Total** | **€1,800,000** | |

The €1.2M optional equity component is reserved for the commercial-layer scale-up (hosted dashboard build-out + EU expansion staff). If awarded only as grant, Phase 3 commercial scale-up is funded via the open-core SaaS revenue trajectory (€400K ARR by Month 24) plus follow-on EU funding (Horizon CL3-ECCC or PT2030 SICE Internacionalização — see §4).

### 3.3 Roadmap to commercialization

| Month | Milestone |
|---|---|
| M0 | EIC Step 1 submission + Sovereign Tech Fund submission in parallel (this submission's pre-week). |
| M2 | Sovereign Tech Fund M1 milestone delivered: production Rust core + Python SDK v0.2 published. |
| M3 | EIC Step 1 decision received. If GO: ANI Voucher Deep Tech filing initiated; Lda incorporation finalised. |
| M4–6 | Sovereign Tech Fund M2 milestone: production transparency log + WASM web verifier. First non-Conforme pilot signed. |
| M6 | EIC Accelerator Step 2 submission (full application). Funding bridge in place. |
| M8 | Sovereign Tech Fund M3 milestone: Article 12 evidence pack + Radically Open Security audit complete. |
| M9 | Step 2 evaluation result; if GO, contract signed by M11. |
| M12 | v1.0 shipped (Apache-2.0). Standards-body submission accepted (CNCF or W3C). 25–40 paying customers. |
| M18 | First enterprise contract (target: tier-2 EU bank). Decision Schema reaches RFC-status in chosen standards body. |
| M24 | €400K ARR. Self-sustaining. EU expansion underway. Phase 3 (federated multi-org logs + EUDI Wallet binding) in design. |

### 3.4 Concurrent funding strategy

Attestly has a deliberate **non-overlapping multi-funder strategy**, all disclosed:

- **Sovereign Tech Fund (€100K)** — funds the open commons artefacts (library, CLI, verifier, spec, security audit). Already in flight, submitted in parallel with this Step 1 application.
- **EIC Accelerator (this submission)** — funds the commercial-layer scale-up, EU market entry, team build-out, standards-body engagement (the layer that sits *on top of* the open commons artefacts). Grant + optional equity component.
- **ANI Voucher Deep Tech (€60K + €10K)** — Portuguese national funding, mechanically unlocked by a GO on this Step 1. Funds the Lda incorporation and initial commercial-layer engineering.
- **NLnet NGI Zero Commons (1 Oct 2026 cycle, ~€50K)** — Phase 3 follow-on funding for the federated multi-org logs + EUDI Wallet binding component. Submitted post-decision on Sovereign Tech Fund + EIC, when Phase 1 evidence is in hand.

The applicant's existing commercial revenue (Conforme — EU short-term rental compliance SaaS) provides decoupling from any single grant outcome. If EIC Step 1 returns NO, the project continues on the Sovereign Tech Fund + Conforme runway; if Sovereign Tech Fund returns NO, the open commons artefacts ship anyway from the EIC commercial layer.

### 3.5 Risks and mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Microsoft AGT extends to cover non-agentic high-risk AI | Medium | Our format is open + neutral and ingests CloudEvents from any source; we frame as the independent verification layer regardless of which framework produced the events. AGT's documented lack of stable schema commitment helps. |
| EU AI Act enforcement delayed | Low | Aug 2 2026 is locked in legislation. Even if national authorities are slow, the regulation is binding. SOC2 + HIPAA framing carries the story even in delay scenarios. |
| GDPR-transparency tension perceived as showstopper | Medium | Privacy posture is front-and-centre in every comms artefact: only hashes are public, payloads stay operator-controlled. Same as Certificate Transparency + Sigstore. Has been peer-reviewed by EU AI Act-literate readers. |
| Single-founder execution risk | Medium-High | Hire co-maintainer by Sovereign Tech Fund M2. Commercial lead by EIC M12. Demonstrated track record of solo-end-to-end shipping (Conforme). |
| Funding gap if EIC NO and STF NO | Low-Medium | Conforme revenue funds continued open-core development; project shipped as Conforme's internal Article 12 feature with later open-source. Reapply STF + NLnet at next cycle. |
| Solo founder burns out | Medium | Hard cap on weekly hours; Conforme provides income decoupling; explicit hand-off planning at each milestone. |

### 3.6 Strategic interest to the EU

- **AI Act enforcement infrastructure**: without open verification, the regulation is downgraded from binding to advisory. Attestly is the missing protocol layer.
- **Digital sovereignty**: a verification layer not controlled by Microsoft, Google, AWS, or any single commercial vendor. The reference implementation lives in Portugal; the spec lives in CNCF or W3C; no US-hyperscaler is in the trust chain.
- **EU competitive position in AI accountability**: first credible open standard published by an EU SME, before US equivalent gets traction. Standards-body capture is asymmetric — first credible implementer becomes the reference.
- **Cross-border interoperability**: a regulator in Lisbon, Madrid, Berlin, or Paris uses the same `attestly verify --bundle` command on the same bundle format from the same standardised wire schema. Single-market enforcement becomes uniform.

---

## Pitch summary

> Attestly is open verification infrastructure for EU AI Act Article 12 evidence. We give every high-risk AI operator a drop-in tamper-evident audit log + public verifier — without exposing decision payloads. Working v0.1 with 20 tests + screencast. Apache-2.0 + CC-BY-4.0. €100k Sovereign Tech Fund in parallel; this submission is for the commercial-layer scale-up to capture the €100M+ EU AI accountability market opening 2 Aug 2026.

---

## Attachments (referenced in submission, hosted on `attestly.dev` post-domain-registration)

- 90-second demo screencast (`attestly-demo.gif`) — working tamper-detection pipeline.
- Pitch deck (10 slides) — innovation, market, team, ask.
- Sovereign Tech Fund proposal (parallel submission, for transparency).
- GitHub repository `github.com/attestly/attestly` (public from submission day 1).
- Founder CV (1 page).
- Letter of support from Conforme (commercial backer + first pilot).

---

*Submission prepared 2026-05-22. Contact: curts152@gmail.com.*
