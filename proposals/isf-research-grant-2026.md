# Internet Society Foundation — Research Grant 2026

**Project**: Attestly — Empirical study of open public-anchor cryptographic infrastructure for evidentiary integrity
**Funder**: Internet Society Foundation — Research Grant Programme 2026
**Funder portal**: https://www.isocfoundation.org/grant-programme/research-grant-programme/
**Submission target**: 2026 application window (open since April 2026)
**Funding ask**: **US $150,000** over 18 months
**Lead applicant**: Curtis (independent researcher, Portugal-based)
**Project repository**: https://github.com/attestly/attestly
**Project website**: https://attestly.org

---

## Honest framing note

The Internet Society Foundation Research Grant has historically funded academic researchers, civil-society research organisations, and independent research institutes. This application is from a **non-academic independent researcher** building open-source infrastructure with applied research dimensions. The proposal is structured to make explicit the methodology, the peer-review outputs, and the collaboration with academic partners, and to be honest about the limits of solo independent research relative to a fully-resourced academic programme.

If ISF prefers to fund this work through an academic intermediary (e.g., a research grant routed through INESC TEC, Católica-Lisbon CRC, or another Portuguese institution willing to act as the fiscal host), we are open to that structure. The applicant would then act as the project's named principal investigator under the academic host's governance.

---

## Research question

**Are open public-anchor cryptographic primitives sufficient — in isolation, or in composition with existing content-provenance and timestamping standards — to deliver evidentiary integrity at the threshold required for high-stakes regulatory enforcement and civil-society legal use?**

The question is empirical, not theoretical. The cryptographic primitives are well-understood (Merkle trees, Ed25519, hash chains). What is unknown is whether their composition into an open commons primitive can survive contact with three distinct adversarial settings:

1. **EU AI Act Article 12 audit-log integrity at regulatory enforcement scale** (regulator-vs-operator threat model).
2. **Court-admissible field-evidence chain-of-custody for civil-society documentation** (prosecutor-vs-defence threat model in human-rights and journalism casework).
3. **Federated multi-organisation transparency log governance** (operator-vs-operator threat model where no single party can be trusted with the audit substrate).

---

## Methodology

The methodology combines **implementation** (the Attestly open-source reference implementation), **empirical evaluation** (against documented adversarial scenarios in each of the three settings), and **peer-reviewable analysis** (papers targeting ACM CCS, USENIX Security, IEEE Security & Privacy).

### Component 1 — Threat-model analysis (months 1-3)

For each of the three settings, document the relevant adversarial threat model in terms compatible with the cryptographic security literature. The deliverable is a formal threat model document for each setting, peer-reviewed in workshop venues (e.g., IEEE EuroS&P workshops, ACM CCS workshops).

### Component 2 — Empirical evaluation (months 4-12)

Implement adversarial test cases against the Attestly reference implementation:

- **Setting 1 (regulatory)**: synthetic high-risk AI system audit-log scenarios, including operator-side tamper attempts at multiple sophistication levels. Measure detection rate of the Attestly verifier against ground-truth tamper events. Cross-reference with documented enforcement gaps in the literature on AI auditing (Citation network: Raji, Bender, Bommasani, Barocas, etc.).
- **Setting 2 (court evidence)**: case-study analysis of existing field-capture chain-of-custody disputes in published human-rights and journalism casework (ICC briefings, EU Court of Human Rights filings, national-level press-freedom cases). Map how an Attestly-anchored evidence chain would have changed each case's evidentiary trajectory. Methodology grounded in legal-technology empirical research conventions (cf. work by Niloufer Selvadurai, Diana Stafford, et al.).
- **Setting 3 (federation)**: implementation of a multi-organisation transparency log federation between three independent participants. Measure governance and protocol resilience against single-organisation withdrawal, censorship, and tamper attempts.

### Component 3 — Public dataset (months 12-15)

The empirical evaluation outputs a public reproducible dataset:

- Synthetic AI-system audit-log scenarios with documented tamper test cases (for Setting 1).
- Anonymised case-study data covering the field-evidence chain-of-custody analysis (for Setting 2).
- Multi-participant federation protocol traces (for Setting 3).

The dataset is released under CC0 with full methodology documentation, in keeping with open-research norms.

### Component 4 — Peer-reviewed publications (months 12-18)

Three target publications over the 18-month research window:

1. *"Open public-anchor primitives for AI Act Article 12 evidentiary integrity"* — target venue USENIX Security 2027 or ACM CCS 2027.
2. *"Evidentiary chain-of-custody for civil-society field capture: an analysis of public-anchor cryptographic primitives in human-rights documentation casework"* — target venue IEEE S&P 2027 or law-and-technology venue (e.g., Berkeley Tech LJ, Harvard JoLT).
3. *"Witness-based federation for multi-organisation transparency logs"* — target venue ACM Conference on Computer and Communications Security 2027 (CCS workshop track).

All publications are dual-published as open-access preprints on arXiv from the day of submission.

---

## Why this question is internet-impact-relevant

ISF's Research Grant programme funds work that *"contributes to understanding the impact of the internet on society."* The research question above addresses three converging pressure points:

1. **EU AI Act enforcement** (Aug 2026): the regulatory framework names audit logging but is silent on integrity. Whether open public-anchor primitives can fill this gap determines whether the regulation is effectively enforceable against well-resourced operators.

2. **Misinformation and content-authenticity** (continuous): C2PA's adoption at scale by major news outlets sits alongside the documented timestamp-substitution flaw. Understanding what additional primitives are required for legal-evidence use is directly relevant to the internet's epistemic resilience.

3. **Civil-society documentation under authoritarian conditions** (continuous): the ability of journalists and human-rights documenters to produce court-admissible evidence under hostile state surveillance bears directly on the internet's role in democratic accountability.

The research is not abstract: each component produces artefacts (specifications, datasets, implementations) that civil-society organisations, regulators, and standards bodies can adopt directly.

---

## Existing artefacts and prior work

The applicant has already shipped a working v0.1 implementation of the Attestly primitives:

- Public Rust workspace (3 crates, ~1,500 LoC) at github.com/attestly/attestly under Apache-2.0.
- 20 tests passing, including 2 end-to-end integration tests covering the full tamper-detection pipeline.
- Reproducible 27-second demo via `cargo build --release && bash examples/demo.sh`.
- Project documentation and architectural rationale at attestly.org.

The research grant funds the **empirical evaluation and peer-reviewed analysis** of these primitives across the three adversarial settings — not the engineering of the primitives themselves, which is in scope for parallel applications to the Sovereign Tech Fund (Phase 1 core engineering) and Open Technology Fund (Phase 2 civil-society integrations).

---

## Academic collaboration

Two Portuguese research institutions have been identified as candidate informal collaborators during the research window:

- **INESC TEC** (Porto) — for the cryptographic methodology peer-review and the federation-protocol component.
- **Católica-Lisbon Centre for Research in Communication, Internet and Innovation (Católica CRC)** — for the legal-technology empirical research component (Setting 2).

Initial outreach is in preparation. No formal partnership is required by ISF Research Grant terms, but the informal collaboration strengthens methodological rigour and provides peer-review pathways for the empirical components.

If ISF prefers to formalise the collaboration into a research grant routed through one of these institutions as fiscal host (with the applicant as named principal investigator), we are open to that structure.

---

## Timeline (18 months)

| Phase | Months | Activities |
|---|---|---|
| Phase 1 | 1-3 | Threat-model documentation for all three settings; workshop submissions |
| Phase 2 | 4-9 | Empirical evaluation, Setting 1 (regulatory) — synthetic scenarios + Attestly evaluation |
| Phase 3 | 7-12 | Empirical evaluation, Setting 2 (court evidence) — case-study analysis |
| Phase 4 | 10-15 | Empirical evaluation, Setting 3 (federation) — multi-participant implementation |
| Phase 5 | 13-15 | Public dataset preparation + release |
| Phase 6 | 12-18 | Three peer-reviewed publications drafted, submitted, revised |

The phases overlap; the work is sized for a single researcher at 80% allocation over 18 months.

---

## Budget (US $150,000)

| Line item | Amount |
|---|---|
| Research lead time — 18 months @ 80% allocation @ $5,500/month equivalent | $80,000 |
| Cryptographic methodology peer-review honoraria (3 reviewers at $4k each) | $12,000 |
| Legal-technology case-study research assistance — part-time support for archival research in human-rights casework | $18,000 |
| Federation protocol implementation — additional engineering support beyond what Attestly Phase 3 covers | $15,000 |
| Public dataset infrastructure — long-term hosting, methodology documentation | $5,000 |
| Travel — IETF + USENIX Security or equivalent conferences for paper presentation (2 trips) | $8,000 |
| Publication open-access fees (3 papers) | $4,500 |
| Project administration | $7,500 |
| **Total** | **$150,000** |

---

## Why ISF, not OTF or STF or NLnet

This is the **research-output** application — peer-reviewed publications, public datasets, methodology rigor. The parallel applications fund the engineering deliverables (STF for core library, OTF for civil-society integrations, NLnet for federation engineering). ISF funds the empirical analysis and peer-reviewable academic outputs that the engineering work makes possible but does not itself produce. The four funders evaluate on independent criteria; the deliverables do not overlap.

---

## Risks and mitigations

1. **Solo independent researcher without academic institutional support.** This is the main risk. Mitigation: explicit openness to academic-host fiscal arrangement if ISF prefers; informal collaboration with INESC TEC and Católica CRC; peer-reviewable outputs structured for the conventions of computer-security and legal-technology venues.

2. **Empirical research on case-study data may produce inconclusive results.** Inconclusive results are still valuable research outputs; the methodology and dataset are useful regardless of whether the empirical conclusions are dramatic.

3. **Publication timelines may slip beyond the 18-month window.** Mitigation: preprint-first publication strategy (arXiv at time of submission to peer-reviewed venue) ensures the research is publicly available before formal acceptance.

4. **Engineering dependencies on parallel Attestly applications.** If STF or NLnet applications do not land, some federation-protocol implementation work needs to be done within the ISF research budget rather than rely on parallel funding. The budget includes a $15k engineering-support line for this contingency.

---

## References

- arXiv [2604.24890](https://arxiv.org/abs/2604.24890) — C2PA timestamp-substitution analysis
- Certificate Transparency: [RFC 9162](https://datatracker.ietf.org/doc/rfc9162/)
- Sigstore: github.com/sigstore — software supply-chain transparency log
- transparency-dev Tessera: github.com/transparency-dev/tessera
- EU AI Act Article 12: https://artificialintelligenceact.eu/article/12/
- eIDAS 2 Article 45l: https://eur-lex.europa.eu/eli/reg/2024/1183
- INESC TEC research interests: https://www.inesctec.pt
- Católica-Lisbon CRC: https://www.fch.lisboa.ucp.pt

---

*Submission prepared 2026-05-25 for the 2026 ISF Research Grant application window. Contact: hello@attestly.org · curts152@gmail.com · Project: https://attestly.org*
