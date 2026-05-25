# Ready-to-send outreach emails

Copy each block into your mail client. Subject lines marked **Subject:**. Send-routing notes follow each email.

Recommended order:

1. Portugal Space — send today (longest lead time, hard ESA BASS deadline 28 Aug)
2. INESC TEC — strongest fit for the Attestly cryptographic-analysis angle
3. Católica CRC — secondary fit, legal-tech framing
4. IT Aveiro — backup option, cryptography group

Send IAPMEI inquiries on separate days (Mon / Tue / Wed of the same week) so responses don't pile up and you can compare offers.

---

## 1. Portugal Space — ESA BASS letter of authorisation

**To**: `info@portugalspace.pt`
**Cc** (optional): the named ESA BASS national delegate if listed at portugalspace.pt/cooperacao-internacional/esa — verify before sending
**Subject**: ESA BASS Kick-Start submission (28 August cut-off) — request for letter of authorisation

---

Caros Senhores da Portugal Space,

I am writing to request a letter of authorisation for the upcoming ESA Business Applications Space Solutions (BASS) Kick-Start submission round, with a cut-off date of 28 August 2026.

I lead **Conforme** (https://conforme.info), a Portuguese SaaS company in production serving short-term-rental compliance for property managers across Portugal. We are preparing a Kick-Start proposal to integrate Copernicus Sentinel-1/-2/-3 imagery and complementary EO data sources (NPP-VIIRS night-lights, ECOSTRESS thermal) into our existing compliance product, specifically for:

1. **Address verification** of registered RNAL properties (cross-referencing self-declared coordinates against cadastral polygons and SAR/optical signatures).
2. **Grey-market occupancy detection** for properties declared inactive but with EO-derivable thermal or activity signatures.

The proposal is for a 6-month feasibility phase, €100,000 total (€75,000 ESA + €25,000 Conforme co-financing from operating revenue). On successful completion, a natural BASS Demonstration follow-on extends the work to other EU Member States under EU Regulation 2024/1028 (the new EU short-term-rental data-collection regulation entering force across 2027).

The letter of authorisation is, as I understand it, a Portugal Space prerequisite for BASS submission as the Portuguese delegation to ESA. I would be grateful to know:

- The current process and expected timeline for issuance.
- Whether you require a meeting or a written technical brief before issuing.
- The format/template you typically use.

The full draft Kick-Start proposal (currently 7 pages) is ready to share at your request. I am happy to come to your offices (Lisbon-based) or to schedule a video call at your convenience to present the work in person.

Thank you for your time.

Com os melhores cumprimentos,

Curtis  
Founder, Conforme · ENI Portugal  
hello@conforme.info · curts152@gmail.com  
https://conforme.info  
+351 [your phone number]

**Send-routing note**: Portugal Space is the national delegation to ESA. The named BASS contact varies by year; check https://portugalspace.pt/cooperacao-internacional/esa for the current programme officer's name and Cc them directly if listed. If no named contact, `info@portugalspace.pt` is the right entry point.

---

## 2. INESC TEC — IAPMEI Vale Inovação partner inquiry

**To**: `comunicacao@inesctec.pt` (general inquiries) — for a faster path, find the head of the Centre for Information and Cyber Security (CTIC) at https://www.inesctec.pt/en/centres/ctic and contact them directly
**Subject**: IAPMEI Vale Inovação research-partnership inquiry — open-source AI Act audit-log infrastructure

---

Dear INESC TEC research team,

I am writing to inquire about a research-partnership opportunity for a small but high-impact R&D project I am preparing for submission under IAPMEI's Vale Inovação Simplificado programme (deadline 30 November 2026).

I lead **Attestly** (https://attestly.org), an open-source Rust library that publishes append-only Merkle-rooted Signed Tree Heads to a public transparency log — applying the Certificate Transparency pattern to the EU AI Act Article 12 audit-log integrity requirement. The project has a working v0.1, 20 tests passing, and is hosted publicly at github.com/attestly/attestly under Apache-2.0. Parallel grant submissions are in preparation to the Sovereign Tech Fund and NLnet NGI Zero Commons.

The R&D scope I would like to work on jointly is **an independent cryptographic analysis of the Attestly transparency-log protocol and Decision Schema**, focused on:

- Adversarial-model documentation (regulator-vs-operator, prosecutor-vs-defence in civil-society casework, operator-vs-operator in federated multi-org logs).
- Empirical evaluation of the tamper-detection pipeline against documented attack classes (specifically against the C2PA timestamp-substitution flaw documented in arXiv 2604.24890).
- Formal review of the Decision Schema v0.1 specification before public freeze.

The IAPMEI Vale Inovação Simplificado is structured as a €5k–€25k engagement with an accredited research entity, reimbursed at 85%. I am writing to ask:

1. Whether INESC TEC is currently on IAPMEI's accredited research-entity list (I believe so, but would like to confirm).
2. Whether the Centre for Information and Cyber Security (or another INESC TEC group with relevant expertise) would be open to a research engagement of this scope and size.
3. What scoping conversation would be most useful as a next step.

I would be glad to share the full Attestly technical documentation, the working demo, and the grant-proposal context at your convenience. Happy to meet in Porto or via video call.

Com os melhores cumprimentos,

Curtis  
Independent researcher / Founder, Attestly  
hello@attestly.org · curts152@gmail.com  
https://attestly.org · https://github.com/attestly/attestly

**Send-routing note**: INESC TEC has multiple centres. The Centre for Information and Cyber Security (CTIC) is the strongest fit for cryptographic-protocol analysis. If your initial reply comes from `comunicacao@inesctec.pt`, ask them to route to CTIC directly.

---

## 3. Católica-Lisbon CRC — IAPMEI Vale Inovação partner inquiry

**To**: `info@research.lisbon.ucp.pt` — for a faster path, identify the relevant programme director at the Católica Centre for Research in Communication, Internet and Innovation (CRC) and Cc them
**Subject**: IAPMEI Vale Inovação research-partnership inquiry — regulatory framework for EU AI Act Article 12 evidence

---

Dear Católica CRC research team,

I am writing to inquire about a research-partnership opportunity under IAPMEI's Vale Inovação Simplificado programme (deadline 30 November 2026).

I lead **Attestly** (https://attestly.org), an open-source library that publishes cryptographically-verifiable Signed Tree Heads to a public transparency log — providing the integrity assurance layer that the EU AI Act mandates (Article 12) but does not technically specify. The project has a working v0.1 at github.com/attestly/attestly under Apache-2.0. Parallel grant submissions are in preparation to the Sovereign Tech Fund, NLnet NGI Zero Commons, and the Open Technology Fund.

The research scope I would like to work on jointly with Católica CRC is **the regulatory and evidentiary-law framing of public-anchor cryptographic primitives** — specifically:

- A position paper analysing how eIDAS 2 Regulation (EU) 2024/1183 Article 45l (Qualified Electronic Ledgers, full implementation December 2026) interacts with the AI Act Article 12 record-keeping obligations.
- Case-study analysis of how Attestly-anchored evidence chains would have changed the evidentiary trajectory of published civil-society and journalism cases (ICC briefings, European Court of Human Rights filings, national-level press-freedom cases).
- Recommendations for Member State implementation guidance on cryptographic-ledger-anchored audit logging under Article 12.

The IAPMEI Vale Inovação Simplificado is a €5k–€25k engagement with an accredited research entity at 85% reimbursement. Católica CRC's combined expertise across communication law, internet governance, and innovation policy is a strong fit for this scope.

I am writing to ask:

1. Whether Católica CRC is on IAPMEI's accredited research-entity list (I believe Católica institutionally is — would like to confirm CRC specifically qualifies).
2. Whether there is academic interest in the regulatory-framework analysis described above as a small-scope research engagement.
3. What a scoping conversation would look like as a next step.

I would be glad to share the full Attestly technical documentation and the parallel-grant context at your convenience. Lisbon-based; happy to meet at Católica or via video call.

Com os melhores cumprimentos,

Curtis  
Independent researcher / Founder, Attestly  
hello@attestly.org · curts152@gmail.com  
https://attestly.org · https://github.com/attestly/attestly

**Send-routing note**: Católica CRC's website lists current researchers and programme themes — pick the closest match (likely an internet-governance or regulatory-tech researcher) and Cc them on the initial email. Generic `info@research.lisbon.ucp.pt` is the entry point if no specific researcher fits cleanly.

---

## 4. Instituto de Telecomunicações (IT Aveiro) — IAPMEI Vale Inovação partner inquiry

**To**: `secretariado@it.pt` (Instituto de Telecomunicações general secretariat) — for a faster path, find the head of the Security and Cryptography group at IT Aveiro and contact them directly
**Subject**: IAPMEI Vale Inovação research-partnership inquiry — cryptographic protocol review for open-source transparency-log library

---

Dear Instituto de Telecomunicações team,

I am writing to inquire about a research-partnership opportunity under IAPMEI's Vale Inovação Simplificado programme (deadline 30 November 2026).

I lead **Attestly** (https://attestly.org), an open-source Rust library that publishes Merkle-rooted Signed Tree Heads to a public transparency log — applying the Certificate Transparency pattern to EU AI Act Article 12 audit-log integrity. The project has a working v0.1 at github.com/attestly/attestly under Apache-2.0, with parallel grant submissions in preparation to the Sovereign Tech Fund, NLnet NGI Zero Commons, and the Open Technology Fund.

The research scope I would like to engage IT Aveiro's Security and Cryptography group on is **a cryptographic protocol review of the Attestly transparency-log and Decision Schema specifications before public v1.0 freeze**:

- Review of the canonical-hashing approach (currently SHA-256 over a domain-separated JSON encoding; we are evaluating a migration to RFC 8785 JCS).
- Review of the Ed25519 signature usage patterns and key-management story (operator-side identity, DID-bound).
- Review of the Merkle tree structure and Signed Tree Head format for compatibility with the IETF C2SP working group's emerging witness-based federation protocols.
- Recommendations for post-quantum migration pathway (relevant to eIDAS 2 Article 45l Qualified Electronic Ledger compliance over the medium term).

The IAPMEI Vale Inovação Simplificado is a €5k–€25k engagement with an accredited research entity at 85% reimbursement. IT Aveiro's combination of cryptography research and applied protocol expertise (you have prior work on PQC migration and on cryptographic-primitive integration into communication protocols) is a strong scope fit.

I am writing to ask:

1. Whether IT Aveiro is on IAPMEI's accredited research-entity list (I believe so).
2. Whether the Security and Cryptography group would be open to a small-scope review engagement of this kind.
3. What scoping conversation would be most useful as a next step.

I would be glad to share the full Attestly technical documentation, the working demo, and the parallel-grant context at your convenience.

Com os melhores cumprimentos,

Curtis  
Independent researcher / Founder, Attestly  
hello@attestly.org · curts152@gmail.com  
https://attestly.org · https://github.com/attestly/attestly

**Send-routing note**: IT (Instituto de Telecomunicações) is a multi-site research institute with branches in Lisbon, Aveiro, Coimbra, Leiria. The Aveiro branch has the strongest cryptography-research output. The general secretariat will route correctly if you mention "Security and Cryptography group, IT Aveiro" in the email.

---

## Tracking the responses

For each email sent, add to your task list:

- Sent date
- Recipient
- Expected first-response timeline (Portugal Space: 1-2 weeks; INESC TEC: 1-2 weeks; Católica CRC: 2-3 weeks; IT Aveiro: 1-2 weeks)
- Followup nudge if no response after the expected window + 1 week

If you'd like, I can convert this into a tracking table that lives alongside the proposals once you've started sending.
