# McGovern AI for Humanity Prize (via MIT Solve) — Application Prep

**Funder**: Patrick J. McGovern Foundation — AI for Humanity Prize, channeled through MIT Solve
**Cycle status**: Current 2026 cycle **closed 21 May 2026**. Next cycle expected Q4 2026 or Q1 2027.
**Prize size**: $200,000
**Funder portal**: https://www.solve.mit.edu (MIT Solve) and https://www.mcgovern.org
**Application type**: Prize, not grant — submission via MIT Solve Challenge
**Lead applicant**: Curtis (Attestly), Portugal-based

---

## Why this is a prep document, not a proposal

The 2026 cycle's submission window closed four days ago. The next cycle's submission window has not yet been announced. This document captures the application framing, target story arcs, and required evidence so that when the cycle reopens the application can be assembled quickly.

The prep work is deliberate: McGovern AI for Humanity selects on narrative fit as much as technical merit, and pre-thinking the narrative arc avoids a rushed application that under-sells Attestly's public-good dimension.

---

## Cycle tracking checklist

- [ ] Subscribe to MIT Solve newsletter for cycle-opening notification
- [ ] Subscribe to Patrick J. McGovern Foundation funding announcements
- [ ] Monthly check (last working day of each month): mcgovern.org/work/data-science-for-good-impact
- [ ] When cycle opens, target submission within 2 weeks of opening (early submissions get more reviewer attention)

---

## Target story arcs (sorted by likely judging panel resonance)

### Arc 1 — Public-interest AI accountability infrastructure

The strongest pitch for the McGovern AI for Humanity Prize is **Attestly as the public-good infrastructure layer that makes AI accountability mechanically possible at scale**.

Hook: *"The EU AI Act mandates audit logs for every high-risk AI system. The Act is silent on whether those logs can be trusted — a gap that today undermines every accountability mechanism the regulation contains. Attestly closes that gap with open, regulator-runnable, cryptographically-verifiable infrastructure that any high-risk AI operator can adopt without commercial dependency."*

Why this lands: McGovern AI for Humanity emphasises AI used for public good and AI accountability — Attestly is **AI accountability infrastructure**, not an AI product. The distinction matters because judges typically see many "AI for good" applications that are themselves AI systems requiring accountability, and few applications that solve the underlying accountability layer.

### Arc 2 — Court-admissible evidence for civil society

The secondary pitch leverages the civil-society / journalism use case (the same framing as the parallel OTF Internet Freedom Fund application).

Hook: *"When a journalist documents protest violence in Belarus, when a human-rights worker captures arbitrary detention in Hong Kong, when an investigator documents atrocities in Sudan — the captured evidence is only as defensible as its cryptographic chain of custody. Attestly is the open, public-anchor cryptographic backend that makes such captured evidence court-admissible against later authenticity disputes."*

Why this resonates: ties Attestly to concrete at-risk-user impact. Less differentiated from OTF but pairs well with the AI accountability arc as a *scale-of-impact* argument (the same primitive serves both regulators and civil society).

### Arc 3 — The "infrastructure not product" framing

The McGovern Foundation tends to fund organisations rather than products. Attestly is positioned as **a public-good commons primitive operated by a non-extractive maintainer**, not as a SaaS product seeking customer-acquisition funding.

Hook: *"Apache-2.0 in perpetuity. No commercial dependency. No customer-acquisition spend. The funding goes entirely to engineering, security review, and ecosystem adoption — not to capturing the market the public good serves."*

### Arc 4 — Foundation primitive for the post-AI-fraud trust era

The category broader than AI Act Article 12 is real: as generative AI makes synthetic documents, fake evidence, and manipulated audit trails trivial to produce, every regulated sector that depends on "single party produces and stores evidence about itself" inherits the same integrity gap that Article 12 names. Attestly is the foundational open primitive other workflows can adopt to close that gap, *without* itself becoming a "trust platform" (every existing player in that space — DocuSign, ID.me, Vanta, ChainIT, OneSpan, Trulioo — is a commercial SaaS with vendor lock-in).

Hook: *"As AI-generated evidence becomes the default and not the exception, the integrity gap Article 12 names will recur in every regulated sector. Attestly is the open foundation other workflows can adopt — the primitive layer underneath the dozen commercial trust platforms that will inevitably build on top."*

Why this is a McGovern-specific framing (and not a general grant framing): the McGovern Foundation funds *organisations* with mission-aligned, society-wide impact. The "foundational primitive other workflows adopt" framing emphasises Attestly's role as upstream public-good infrastructure rather than a competing market entrant. **Do not use this framing in NLnet/STF/OTF/ISF applications** — those funders evaluate on specific deliverable scope, and the broader narrative dilutes the focus they want.

---

## Required evidence to assemble before the cycle opens

The prize requires submitters to demonstrate (per typical MIT Solve form):

- **Working prototype or live deployment**: ✅ already exists — github.com/attestly/attestly, demo at attestly.org, 20 tests passing.
- **Adoption signal**: 🔲 in progress — Tella integration adapter under development for OTF; IBA outreach in progress; eIDAS 2 binding for Phase 3 NLnet. By Q4 2026 / Q1 2027, expect at least one named adopter or pilot.
- **Public-good commitment**: ✅ Apache-2.0 + CC-BY-4.0 licensing locked from day 1.
- **Team / governance plan**: 🔲 in progress — multi-maintainer onboarding by NLnet Phase 3.
- **Measurable impact projection**: 🔲 to draft — quantify reach via Tella user base + RNAL-equivalent registries across EU Member States.
- **Endorsements / letters of support**: 🔲 if possible — IBA partnership outreach (M3 of OTF), academic collaboration with INESC TEC or Católica CRC (from ISF Research Grant), Conforme commercial customers (operational adopters).

---

## Application content (to be drafted when cycle opens)

The MIT Solve application form typically asks:

1. **Project name + tagline** (~150 chars)
2. **What problem are you solving?** (~300 words) → use Arc 1
3. **Your solution** (~300 words) → describe the three primitives + the public-anchor model
4. **Beneficiaries** → high-risk AI operators (EU and globally), civil-society capture-tool users, regulators, courts
5. **Theory of change** → drop-in compliance → ecosystem adoption → regulatory enforcement capacity → measurable AI accountability outcomes
6. **Why now?** → EU AI Act Aug 2026 + eIDAS 2 Dec 2026 regulatory windows + civil-society demand
7. **Team + organisation**
8. **Funding use** ($200k prize): 70% engineering and security review, 20% ecosystem adoption, 10% community + governance
9. **Measurement plan** → audit logs per EU Member State adoption, civil-society capture-tool integrations, IETF spec adoption, federation participants

Drafts of sections 1-3 + 5 + 6 should be derived from the OTF concept note + STF proposal language, lightly re-tuned for McGovern's selection criteria.

---

## Key narrative differentiation from parallel applications

| Funder | Primary narrative |
|---|---|
| **STF** | Open digital base infrastructure (plumbing thesis) |
| **OTF** | Civil-society evidence integrity (at-risk-user thesis) |
| **NLnet NGI0** | Federated commons primitive (decentralisation thesis) |
| **ISF Research Grant** | Empirical study (peer-reviewed research thesis) |
| **EIC Step 1** | Deep-tech commercial venture (innovation thesis, mechanical path to ANI Voucher) |
| **McGovern AI for Humanity** | AI accountability infrastructure as public good (mission alignment with foundation thesis) |

The McGovern application leans hardest into the *mission alignment* dimension. The technical work is the same; the narrative is the most explicitly mission-aligned of the six.

---

## Open questions

1. **Should McGovern be approached BEFORE the formal MIT Solve cycle opens?** The Patrick J. McGovern Foundation does separate direct grant-making for organisations they discover through other channels (recent precedents: GovAI, Stanford HAI, etc.). It may be worth identifying the relevant programme officer at McGovern (likely the Data and Society or AI for Humanity programme leads) and sending a cold introduction with the OTF concept note attached. The MIT Solve prize would then be a secondary path if the direct channel does not open.

2. **Should we pre-build a McGovern-specific landing page section at attestly.org?** A dedicated "Mission alignment" page explaining the public-good commitment in foundation-friendly language. To consider after the OTF and STF outcomes are clearer.

---

## References

- McGovern Foundation: https://www.mcgovern.org
- MIT Solve: https://www.solve.mit.edu
- 2026 cycle dates archived: (capture from web archive once available)
- Adjacent Foundation work for context: Foundation funded GovAI's "Accountability for AI: The Case for Transparency Logs" research stream in 2025 (cite-on-application if true; verify before referencing)

---

*Prep document prepared 2026-05-25. Update when next cycle opens. Contact: hello@attestly.org · curts152@gmail.com*
