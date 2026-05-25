# NLnet NGI Zero Commons — Phase 3 Proposal

**Project**: Attestly — Federated multi-organisation transparency logs and EUDI Wallet operator identity
**Funder**: NLnet — NGI Zero Commons Fund
**Funder portal**: https://nlnet.nl/commonsfund/
**Submission target**: October 2026 cycle (cut-off date as announced by NLnet)
**Funding ask**: **€50,000** (NGI0 Commons cash maximum)
**Plus in-kind support requested**: NGI0 Review for independent security review, accessibility review, and PT/ES localisation
**Lead applicant**: Curtis (Portugal-based, sole technical lead)
**Project repository**: https://github.com/attestly/attestly (public, Apache-2.0)
**Project website**: https://attestly.org

---

## Thematic call

NGI Zero Commons Fund — open digital base technology for a more resilient and democratic internet.

## Project tagline (≤ 200 chars)

Open federated multi-organisation transparency logs, bound to EUDI Wallet operator identity, for cryptographic evidence integrity across civil-society, journalism, and regulated-AI ecosystems.

## Abstract (~300 words)

Attestly is a small Rust library and verifier protocol that publishes append-only Merkle-rooted Signed Tree Heads to a public transparency log, applying the Certificate Transparency pattern (RFC 9162) to AI accountability and field-evidence integrity. Phases 1 and 2 of the project (core library, EU AI Act Article 12 base infrastructure, civil-society integrations) are addressed by parallel applications to the Sovereign Tech Fund and Open Technology Fund respectively, and are not duplicated in this submission. **Phase 3 — funded by this NLnet NGI0 Commons application — addresses the two remaining gaps before Attestly becomes a usable commons primitive at ecosystem scale:**

**Gap 1 — federated multi-organisation logs.** A single-organisation transparency log only protects evidence integrity within that organisation's trust boundary. For an ecosystem-wide commons primitive, multiple organisations (e.g., a human-rights documentation network, a consortium of AI operators, a federation of capture-tool deployments) must be able to *share* a transparency log without trusting each other or a single operator. Phase 3 implements the cross-organisation log-federation protocol and adopts Google's `transparency-dev` Tessera implementation as the production backend.

**Gap 2 — EUDI Wallet operator identity.** Attestly's current operator-identity primitive is `did:web` — a self-asserted DID method, sufficient for prototype but insufficient for regulator-grade attestation. Phase 3 binds operator identity to the EU Digital Identity Wallet (eIDAS 2 framework, with Personal Identification Data and Qualified Electronic Attribute attestations), with selective-disclosure primitives so a single Signed Tree Head can prove operator identity at the precision a regulator or court requires without over-disclosing.

The deliverables are open-source (Apache-2.0), framework-agnostic, and embeddable in any field-capture tool, AI-system audit pipeline, or general-purpose ledger application. The work is technically conservative — established cryptographic primitives, well-understood transparency-log patterns, EU-standardised wallet binding — and the project demonstrates a working v0.1 today.

---

## Have you been involved with projects or organisations relevant to this project before?

Yes. The applicant builds and maintains **Conforme** (https://conforme.info), an EU short-term-rental compliance SaaS in production with paying customers across Portugal. Conforme is the applicant's primary commercial activity and the source of demonstrated track-record in regulated-industry software. Conforme also serves as the natural first pilot for Attestly — its NRUA registration wizard produces structured decision events that are Article 12-shaped and benefit from tamper-evident audit primitives.

The applicant has previously worked on multi-agent AI orchestration systems, Solana / blockchain infrastructure, and tooling for cross-jurisdictional regulatory compliance. Attestly emerges from the observation, made during Conforme's compliance work, that Article 12 of the EU AI Act mandates audit logging but is silent on log integrity — and that the same observation applies, recursively, to every domain where a single party both produces and stores evidence about its own conduct.

The project lead has not previously received an NLnet grant. A parallel NGI0 Commons application from the applicant (for Conforme) is in evaluation as of submission date.

---

## Requested amount

**€50,000** cash, plus in-kind support requested separately via NGI0 Review.

## Explain what the requested budget will be used for

Phase 3 is sized as a 14-week build with the lead developer at 60% allocation and one paid-honorarium co-maintainer at 20% (the multi-maintainer governance commitment from Phases 1 and 2 made concrete in Phase 3).

| Line item | Amount |
|---|---|
| Core engineering — federated log protocol, Tessera production integration, multi-org coordination layer (10 weeks @ €3,500/week-equivalent, lead developer 60% × €5,800/week) | €35,000 |
| EUDI Wallet binding — Personal Identification Data + Qualified Electronic Attribute attestation, selective-disclosure primitives (4 weeks @ €3,000/week-equivalent) | €12,000 |
| Co-maintainer honorarium (paid contributor at 20% for 14 weeks, multi-maintainer governance) | €3,000 |
| **Total** | **€50,000** |

Production infrastructure (Tessera log hosting, CDN), professional translation, and independent security review are requested as **NGI0 Review in-kind support** rather than cash items, in line with NLnet's split between cash and in-kind support.

---

## Compatibility with our values

### Free and open-source

All Phase 3 artefacts ship under **Apache-2.0** (code) and **CC-BY-4.0** (specifications), matching the licensing locked in Phases 1 and 2. The repository (github.com/attestly/attestly) is public from day 1. Specifications are published openly at spec.attestly.org (CC-BY-4.0). There is no closed core, no contributor-license-agreement requirement, and no time-limited evaluation tier. The hosted commercial dashboard layer the applicant maintains for sustainability runs in a separate proprietary codebase and never imports private functionality back into the open libraries.

### Open standards

Phase 3 commits to open standards engagement:
- **Decision Schema** is submitted to the CNCF CloudEvents working group or W3C Community Group as a candidate profile (decision documented in `docs/standards-strategy.md`).
- **Verifier Protocol** is submitted to the IETF SCITT working group as an Internet-Draft.
- **EUDI Wallet binding** follows the eIDAS 2 ARF (Architecture and Reference Framework) and the OpenID4VP / OpenID4VCI specifications. No proprietary identity extensions.

### Privacy

Attestly is architected so that **only short cryptographic commitments touch public infrastructure** — never the underlying decisions, photos, locations, or metadata. The Signed Tree Head is a Merkle root and an Ed25519 signature; nothing about the evidence content is recoverable from public observation. This is structurally GDPR-compatible by construction, not by configuration.

Phase 3 deepens the privacy posture by:
- Implementing **selective-disclosure** primitives so an operator can prove a specific claim about themselves (e.g., "I am a registered EU operator with NIPC X under economic activity Y") without disclosing unrelated identity attributes.
- Implementing **per-record privacy envelopes** so a single transparency-log entry can carry zero metadata visible to log operators or external observers (the log sees only `(commitment_hash, signature)` pairs).

### Resilience

Federated multi-organisation logs eliminate the single-operator-trust assumption that every prior phase of Attestly carries. With federation, no single organisation (including the applicant's) can withdraw, censor, or alter the audit substrate without external observers detecting the inconsistency. The architecture is the same logic that makes Certificate Transparency resilient to a single CA's misbehaviour.

---

## Comparison with existing efforts

### Single-organisation transparency logs

**Google `transparency-dev` Tessera** (the supported successor to Trillian) is the reference single-organisation log implementation. Attestly Phase 3 adopts Tessera as the log backend rather than building a competing log. The contribution is the *federation protocol* on top of Tessera, not the log itself.

### Federated logging systems

**Sigstore Rekor** federates transparency logs across the software supply chain, with a community-of-trust governance model. The pattern is directly applicable. Attestly Phase 3's contribution is the binding to evidence-integrity use cases (field capture, AI accountability) rather than software supply chain, plus the EUDI Wallet operator-identity primitive that Rekor does not currently address.

### EUDI Wallet relying-party integrations

The European Commission ships seven official repositories under `eu-digital-identity-wallet` on GitHub (verifier endpoint, web verifier, multiplatform verifier, Kotlin/Python servers, OpenID4VP libs). Multiple commercial vendors (Sphereon, Procivis, walt.id, Lissi, Gataca, eIDAS Pro) ship verifier-as-a-service products. The applicant's Phase 3 binding is **not** a competing EUDI Wallet verifier — it is a *consumer* of EUDI Wallet attestations bound to the transparency-log operator-identity primitive. The architectural delta is small (which is the point) and the integration leverages existing open EUDI Wallet libraries rather than re-implementing them.

### AI audit-log governance

**Microsoft Agent Governance Toolkit** (April 2026, MIT-licensed) is the closest market reference. AGT scopes to *agentic* AI within Microsoft's framework stack and provides in-process verification only. Attestly is framework-agnostic and provides regulator-runnable verification — a distinct contribution mapped in detail in `docs/agt-comparison.md` in the repository.

---

## Concrete technical challenges

The work is technically conservative but not trivial. Three challenges shape Phase 3:

1. **Federation without coordination overhead.** Multi-organisation log federation typically requires either a shared operator (re-introducing the single-trust assumption) or a complex consensus protocol (re-introducing the operational overhead that motivated the single-operator model in the first place). Attestly's approach is *witness-based*: each organisation runs its own log, but cross-attests other organisations' tree heads via a periodic witness exchange. This is the same pattern emerging in the IETF C2SP working group for software-supply-chain federation, and we will closely track that group's protocol decisions to avoid divergence.

2. **EUDI Wallet binding before all member states have shipped.** The eIDAS 2 timeline puts full EUDI Wallet implementation at December 2026, but real-world wallet rollout per Member State will lag. Phase 3 must work both with a fully-deployed EUDI Wallet *and* with a fallback to ARF-compliant non-state attestations during the transition period. This is a graceful-degradation problem with multiple correct answers — the design decision is documented in `docs/eudi-wallet-binding-strategy.md` and will be reviewed with relevant EU bodies during the grant period.

3. **Selective disclosure without leaking the disclosure surface.** A naive selective-disclosure implementation leaks information about the *set* of attestations the operator could have disclosed (i.e., the absence of certain claims is itself a claim). Phase 3 uses zero-knowledge predicate proofs (Bulletproofs or BBS+ signatures, dependent on EUDI Wallet adoption) to address this. The cryptographic choices are well-understood; the implementation discipline is the engineering work.

---

## Ecosystem engagement

Phase 3 produces concrete ecosystem outputs:

- **IETF SCITT working group**: Internet-Draft submission for the Verifier Protocol and Federation Protocol.
- **IETF C2SP working group**: position document on witness-based federation as it applies to evidence-integrity use cases beyond software supply chain.
- **EU Digital Identity Wallet open-source community** (`eu-digital-identity-wallet` GitHub org): pull request adding Attestly as a downstream consumer reference in their consumer-integration catalogue.
- **Tella, ProofMode, eyeWitness** (civil-society capture-tool maintainers): Phase 3 federation protocol enables these tools to share a transparency-log substrate without trusting each other or a single operator. Outreach to each is documented in M3 of the OTF Phase 2 application and continues into Phase 3.
- **FOSDEM 2027** (Brussels): talk submission on federated transparency logs for evidence integrity, planned as a co-presentation with one ecosystem partner (Tella or Sigstore Rekor maintainer).
- **academic research community**: position paper on Attestly's federation protocol drafted for ACM CCS 2027 submission (post-grant), with the goal of moving the architecture into peer-reviewed venues.

---

## Concrete results at the end of the grant period

The deliverables exist as testable artefacts in the public repository, not as planning documents:

- **`attestly-federation` crate**: Rust implementation of the witness-based federation protocol, with at least two reference participants (the applicant's primary log and one ecosystem-partner log).
- **`attestly-eudi-wallet` crate**: EUDI Wallet binding for operator identity, with the OpenID4VP / OpenID4VCI flows implemented and tested against the EU Commission's reference wallet.
- **`spec/federation-v0.1.md`**: federation protocol specification (CC-BY-4.0).
- **`spec/eudi-binding-v0.1.md`**: EUDI Wallet binding specification (CC-BY-4.0).
- **IETF Internet-Draft**: SCITT WG submission of the Verifier Protocol with federation extensions.
- **Multi-participant federation demo**: a reproducible test deployment where two independent log operators cross-attest each other's tree heads, and a third-party verifier confirms the cross-attestation chain.
- **Independent security review** completed via NGI0 Review in-kind support, with all blocker findings resolved.

All artefacts are runnable, reviewable, and embeddable by any third party from day 1.

---

## Sustainability beyond the grant

Attestly's continued maintenance is decoupled from grant cycles by three independent channels:

1. **Conforme commercial revenue** — the applicant's existing SaaS provides primary income, ensuring the lead developer's economic ability to maintain Attestly post-grant.
2. **A separate hosted commercial dashboard** built on top of the open libraries (sustainability layer, never replacing the open commons).
3. **Subsequent grant submissions** for further-phase work (e.g., Phase 4 selective-disclosure deepening, Phase 5 hardware-security-module integrations). Each phase is scoped to a discrete grant cycle rather than aggregating into a single mega-application.

---

## Comparison disclosure (parallel applications)

The applicant has parallel grant applications in flight or in preparation:

- **Sovereign Tech Fund**: Attestly Phase 1 — core library and EU AI Act Article 12 base infrastructure.
- **Open Technology Fund Internet Freedom Fund**: Attestly Phase 2 — civil-society integrations (Tella adapter, browser verifier, IETF SCITT engagement, IBA outreach).
- **EIC Accelerator Step 1 → ANI Voucher Deep Tech (Portugal)**: a separate commercial dashboard layer that sits *on top of* the open library, never duplicating Attestly's grant-funded deliverables.
- **Internet Society Foundation Research Grant 2026**: research framing of the Attestly architecture for academic publication.
- A **separate Conforme** application is also in NLnet NGI0 Commons evaluation; the applications are scoped to disjoint projects.

The boundaries between Phase 3 (this submission) and the other applications are documented in `proposals/README.md`. No deliverable is funded twice. The applicant will withdraw any application or rescope any deliverable that NLnet identifies as overlapping with another funded grant.

---

## References

- C2PA timestamp-substitution and expiry: arXiv [2604.24890](https://arxiv.org/abs/2604.24890)
- eIDAS 2 Regulation (EU) [2024/1183](https://eur-lex.europa.eu/eli/reg/2024/1183), Articles 45l and ARF
- Certificate Transparency: [RFC 9162](https://datatracker.ietf.org/doc/rfc9162/)
- Sigstore Rekor: github.com/sigstore/rekor
- transparency-dev Tessera: github.com/transparency-dev/tessera
- IETF SCITT: https://datatracker.ietf.org/wg/scitt/about/
- IETF C2SP: https://datatracker.ietf.org/wg/c2sp/about/
- EU Digital Identity Wallet ARF: https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework

---

*Submission prepared 2026-05-25 for the October 2026 NLnet NGI0 Commons cycle. Contact: hello@attestly.org · curts152@gmail.com · Project: https://attestly.org · Repository: github.com/attestly/attestly*
