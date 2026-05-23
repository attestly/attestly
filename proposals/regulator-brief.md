# Attestly — brief for AI Act market-surveillance and regulatory authorities

> One-page summary. Audience: regulator, supervisor, market-surveillance
> authority, judicial body, or any party who needs to verify an EU AI Act
> Article 12 log without trusting the operator.

---

**Attestly is open verification infrastructure for AI Act Article 12 evidence.** It lets you — the regulator — confirm that an operator's audit trail has not been altered after the fact, using a single command-line tool, without any access to the operator's systems.

---

## What the regulator needs to know

The EU AI Act mandates that every operator of a high-risk AI system per Annex III (credit, hiring, biometric, employment, essential services, law enforcement, migration, justice) maintains automatic logs of operation. The Act sets the obligation. It does not set the integrity standard.

In practical terms: today, when you request Article 12 evidence from an operator under your jurisdiction, you receive what the operator chooses to show you. There is no technical means by which you can confirm that the record presented is the same record that existed at the time the AI made the decision.

Attestly closes this gap as **open public infrastructure**, not a vendor product.

## How it works for the regulator, in three steps

1. **The operator publishes signed cryptographic commitments to a public location.** These commitments are short hashes that prove "as of timestamp X, the audit log of decisions through Y has root hash Z, signed by my organisational key". They contain no personal data, no decision content — only the cryptographic fingerprint. (Same construction as Certificate Transparency in the web's PKI.)

2. **When you request evidence on a specific decision (say, decision #4,217 made on 12 June 2026 at 14:23 UTC):** the operator exports a small bundle (~3 KB) containing the decision payload, an inclusion proof, and a reference to the published commitment.

3. **You run `attestly verify --bundle <file>` on your own machine.** The verifier:
   - recomputes the canonical hash of the decision payload presented to you;
   - checks the inclusion proof against the operator's previously-published Signed Tree Head;
   - verifies the Ed25519 signature on that Signed Tree Head against the operator's published public key (a standard `did:web` document).

If the operator altered the decision payload after publication, the verifier reports **`[FAIL] TAMPERED`** with the exact cryptographic mismatch. The operator cannot pass this verifier without producing the original record.

## What the regulator does *not* need to do

- Install Attestly. The library is for operators. The verifier is a single open-source tool you run on your own laptop.
- Trust the operator. Verification is mathematical, not procedural.
- Access the operator's database. The verifier reads only the exported bundle plus the public commitments.
- Build infrastructure. Public commitments are hosted by the operator on the open web (or, in production, on a community-operated transparency log similar to Sigstore Rekor).

## What about privacy

**Only cryptographic commitments are published**. No decision payload, no personal data, no model input leaves the operator's environment by default. The public log contains short hash digests with no capability for re-identification.

This separation — public commitments, private payloads — is the same pattern Certificate Transparency uses for TLS and Sigstore uses for software supply chains. It is **GDPR-compatible by construction**. The CNPD (Portuguese Data Protection Commissioner) has been notified of the design for informal review.

## What is currently shipping

- Working open-source Rust library, with 20 automated tests and a full tamper-detection end-to-end demonstration.
- Specifications versioned and published (Decision Schema v0.1, Checkpoint Format v0.1, Verifier Protocol v0.1).
- All artefacts under Apache-2.0 (code) and CC-BY-4.0 (specifications), in perpetuity. No vendor lock-in. No commercial layer in the verification path.
- A 27-second screencast demonstrating tamper detection working end-to-end: github.com/attestly/attestly.

By Q3 2026: a WASM-based web verifier you can run in any browser; an Article 12 evidence-pack export tool with all required artefacts in a single ZIP; a working bridge from Microsoft Agent Governance Toolkit's event format; and an independent security review by Radically Open Security (or equivalent).

## Standards posture

The wire formats are being proposed to the CNCF CloudEvents Working Group as profile candidates, with parallel engagement at IETF SCITT and W3C. Attestly is intended to be **one of several compatible implementations** of an open standard — not a proprietary platform.

## What this project would value from a regulator

Three things, ranked by impact:

1. **Informal technical review** of the verifier protocol and the GDPR posture, with feedback on whether the design choices match your inspection workflow.
2. **A statement of interest** — even a paragraph — that we can cite in our Sovereign Tech Fund and EIC Accelerator applications, acknowledging that the work addresses a problem your authority recognises. This is regulator-side validation, not endorsement of a product.
3. **An introduction to peer authorities** in other Member States, EDPB, or ENISA networks working on AI Act implementation.

---

**Contact**:

Curtis (project lead, Portugal-based) — curts152@gmail.com — github.com/attestly/attestly

Available for a 30-minute video call in English or Portuguese at your convenience. Documentation can be provided in additional languages on request.

---

*Prepared 22 May 2026. This document is intentionally short. Detailed technical materials, formal specifications, and the Sovereign Tech Fund submission text are available on request or via the repository above.*
