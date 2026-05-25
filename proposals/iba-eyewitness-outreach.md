# Outreach letter — International Bar Association, eyeWitness to Atrocities team

**Recipient**: Carrie Bowker, Director, eyeWitness to Atrocities (International Bar Association)
**Routing**: `general@eyewitness.global` with subject line including "FAO Carrie Bowker, Director"; Cc `techsupport@eyewitness.global`. eyeWitness publishes no individual staff emails — only four functional inboxes — so the FAO subject line is how it routes internally.
**From**: Curtis (Attestly), curtis@attestly.org
**Subject**: FAO Carrie Bowker, Director — open-source public-anchor primitive for eyeWitness v2 consideration

---

Dear Ms Bowker,

I'm writing to introduce **Attestly** — an open-source Rust library and verifier protocol that publishes append-only Merkle-rooted Signed Tree Heads to a public transparency log for cryptographic evidence integrity. It's permissively licensed (Apache-2.0) and designed as a framework-agnostic primitive that any capture tool can adopt without commercial dependency. The project page is at https://attestly.org and the working code is at github.com/attestly/attestly.

I'm reaching out because eyeWitness has been the most credible institutional benchmark for court-admissible field evidence for the better part of a decade, and the chain-of-custody work the team has done through LexisNexis sets the bar against which every newer open-source attempt is measured. As the field-capture ecosystem broadens — Tella, ProofMode, regional civil-society networks — there's a growing conversation about how an open, publicly-verifiable anchoring layer could complement rather than compete with the existing eyeWitness architecture. I'd value your team's view on whether such a layer is desirable, what its design constraints should be, and where it would be most useful.

A few specifics that might be relevant:

- The eIDAS 2 Regulation (EU 2024/1183, Article 45l) recognises Qualified Electronic Ledgers with a presumption of authenticity from December 2026. Attestly is architected to be a candidate reference implementation under that framework — not exclusively, and not pre-approved by any court, but architecturally aligned.
- The verification model is genuinely independent: a prosecutor receives a single evidence bundle and runs a 200 kB browser-WASM verifier locally. No vendor cooperation required, no server round-trip, no metadata leakage.
- The published commitments contain only short Merkle roots and signatures — never the underlying captures, locations, or metadata. The privacy posture is designed to support sensitive-source casework.

A 27-second screencast demonstrating the full tamper-detection pipeline is at https://attestly.org. The end-to-end demo is reproducible by any reviewer in about five minutes from the public repository.

I'm not asking for a partnership commitment. What I'd value, if it's possible, is an honest written critique from whoever on your team thinks about long-term technical architecture for eyeWitness — a short reply by email, no matter how brief, would be much more useful than building further without testing the approach against the people whose threat model has been thought through most carefully.

I work entirely async and would prefer to keep this exchange to email; happy to send a longer technical brief in writing if it's useful for review.

Thank you for the work you do. Whatever the outcome of this exchange, the public verifier and the open Decision Schema specification will remain Apache-2.0 / CC-BY-4.0 in perpetuity and free for any organisation to adopt.

With respect,

Curtis
Attestly · Lisbon, Portugal
curtis@attestly.org · https://attestly.org · github.com/attestly/attestly

---

## Send notes (for Curtis when actually sending)

- **Best initial routing**: send to `general@eyewitness.global` with `techsupport@eyewitness.global` Cc'd. Subject line must include "FAO Carrie Bowker, Director" — eyeWitness publishes no individual staff emails, only functional inboxes, so the FAO is how the message reaches the right desk.
- **Do not address Wendy Betts** — she previously led eyeWitness but Carrie Bowker is the current Director as of the 2025-2026 eyewitness.global staff page. Verified by the IBA's own ten-year anniversary piece.
- **Alternative routing**: contact form at https://www.eyewitness.global/connect, paste this text in the message body and tag Carrie Bowker in the subject.
- **Followup**: if no response within 3 weeks, a single polite nudge to Isabelle Bienfait (Programme Co-ordinator, joined 2024) via the same `general@` inbox FAO her by name.
- **Do not** send before the Attestly v0.1 demo is publicly viewable at attestly.org and the GitHub repo is public — both prerequisites are now met as of 2026-05-25.
- **Do** include the OTF concept note and STF proposal status in any follow-up, but not in the first message — keep the first contact short and let them ask.
