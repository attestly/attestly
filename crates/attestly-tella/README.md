# attestly-tella

Adapter that integrates Attestly's public-anchor cryptographic integrity layer
into the Tella (Horizontal) secure-capture upload pipeline.

> **Status: scaffold (v0.0).** This crate defines the API surface and type
> contract for the Tella integration. The integration logic is stubbed for
> a downstream OTF Internet Freedom Fund milestone (M1 — Tella adapter
> deliverable). The crate compiles, the public API is testable, and the
> integration contract is documented — but it has not yet been wired up to
> a live Tella deployment. Adoption discussions with Tella maintainers
> (Horizontal) are pending.

## What this crate does

Tella ([github.com/Horizontal-org/Tella-Android](https://github.com/Horizontal-org/Tella-Android),
[Tella-iOS](https://github.com/Horizontal-org/Tella-iOS)) is the most-deployed
open-source secure-capture mobile app for journalists, human-rights documenters,
and civil-society investigators working in restricted-press and authoritarian
environments. It handles encrypted-at-rest mobile capture, secure upload to
org-controlled Uwazi/ODK/Nextcloud backends, and metadata capture — but it
ships **no integrity layer** for evidentiary chain-of-custody.

`attestly-tella` adds the integrity layer **without modifying Tella itself**.
The adapter sits between Tella's existing upload payload and the destination
backend (Uwazi / ODK / Nextcloud / etc.). For each upload it:

1. Canonicalises the upload payload (file content + Tella metadata).
2. Computes a SHA-256 hash over the canonical form.
3. Appends a `FieldEvidenceEvent` to the operator-organisation's Attestly
   ledger.
4. Returns an `AttestlyReceipt` containing the event id, the ledger position,
   and (once a Signed Tree Head has been published covering this event) an
   inclusion proof against the public transparency log.
5. The receipt is included in the destination backend's record so that, years
   later, a prosecutor or human-rights lawyer can drag the record into the
   Attestly browser verifier and confirm the evidence has not been altered
   since publication.

The **media itself stays private** to the operator-organisation. Only short
cryptographic commitments (Merkle roots, Ed25519 signatures) touch public
infrastructure. Same architecture as Certificate Transparency for TLS.

## API surface (current scaffold)

```rust
use attestly_tella::{TellaAdapter, FieldEvidenceEvent, TellaUpload, AttestlyReceipt};

let adapter = TellaAdapter::new(ledger, org_identity);

let upload = TellaUpload {
    file_bytes: tella_blob,
    metadata: tella_metadata_json,
    captured_at: timestamp,
    source_tool: "tella-android".into(),
    capture_context: Some(...),
};

let receipt: AttestlyReceipt = adapter.attest(upload)?;
// receipt.event_id, receipt.append_position, receipt.signed_checkpoint_ref
```

The full API and protocol notes are in [`docs/protocol.md`](docs/protocol.md)
(to be written for OTF M1).

## What is *not* in this scaffold

The following are explicitly out-of-scope for the v0.0 scaffold and will be
implemented in OTF M1 deliverables:

- **Live Tella deployment integration**. We have not yet shipped a working
  pull request to `Horizontal-org/Tella-Android` or its server-side
  counterparts. The adapter is a library that any Tella deployment can adopt;
  upstream merge is best-effort.
- **Uwazi/ODK/Nextcloud sidecar binary**. A separate `attestly-tella-sidecar`
  binary will wrap this library for deployments that prefer a sidecar service
  rather than embedding the library directly.
- **Browser WASM verifier integration**. The browser verifier consumes the
  `AttestlyReceipt` format defined here; the verifier itself is in
  `attestly-verifier` and the WASM build is a separate OTF M2 deliverable.

## License

Apache-2.0, matching the rest of the Attestly workspace.

## Acknowledgements

This adapter is being developed under a planned [OTF Internet Freedom Fund](https://www.opentech.fund/funds/internet-freedom-fund/)
grant proposal (Phase 2 of the Attestly project). It is published in
scaffold form ahead of grant approval so that potential ecosystem partners
(Horizontal, eyeWitness/IBA, ProofMode/Guardian Project, Witness) can
inspect the API contract and provide feedback ahead of the engineering
build.
