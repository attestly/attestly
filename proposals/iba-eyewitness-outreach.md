# Outreach letter — International Bar Association, eyeWitness to Atrocities team

**Recipient**: eyeWitness to Atrocities team (International Bar Association)
**Routing options**: info@eyewitness.global · partnerships@int-bar.org · contact form at eyewitness.global/contact-us
**From**: Curtis (Attestly), curtis@attestly.org
**Subject**: Public-anchor cryptographic integrity layer — open-source primitive for eyeWitness v2 consideration

---

Dear eyeWitness team,

I'm writing to introduce **Attestly** — an open-source Rust library and verifier protocol that publishes append-only Merkle-rooted Signed Tree Heads to a public transparency log for cryptographic evidence integrity. It's permissively licensed (Apache-2.0) and designed as a framework-agnostic primitive that any capture tool can adopt without commercial dependency. The project page is at https://attestly.org and the working code is at github.com/attestly/attestly.

I'm reaching out because eyeWitness has been the most credible institutional benchmark for court-admissible field evidence for the better part of a decade, and the chain-of-custody work the team has done through LexisNexis sets the bar against which every newer open-source attempt is measured. As the field-capture ecosystem broadens — Tella, ProofMode, regional civil-society networks — there's a growing conversation about how an open, publicly-verifiable anchoring layer could complement rather than compete with the existing eyeWitness architecture. I'd value your team's view on whether such a layer is desirable, what its design constraints should be, and where it would be most useful.

A few specifics that might be relevant:

- The eIDAS 2 Regulation (EU 2024/1183, Article 45l) recognises Qualified Electronic Ledgers with a presumption of authenticity from December 2026. Attestly is architected to be a candidate reference implementation under that framework — not exclusively, and not pre-approved by any court, but architecturally aligned.
- The verification model is genuinely independent: a prosecutor receives a single evidence bundle and runs a 200 kB browser-WASM verifier locally. No vendor cooperation required, no server round-trip, no metadata leakage.
- The published commitments contain only short Merkle roots and signatures — never the underlying captures, locations, or metadata. The privacy posture is designed to support sensitive-source casework.

A 27-second screencast demonstrating the full tamper-detection pipeline is at https://attestly.org. The end-to-end demo is reproducible by any reviewer in about five minutes from the public repository.

I'm not asking for a partnership commitment. What I'd value, if it's possible, is a 30-minute conversation with whoever on your team thinks about long-term technical architecture for eyeWitness. The conversation might lead nowhere, and that's fine; I'd rather hear an honest critique of the approach now than build further without testing it against the people whose threat model has been thought through most carefully.

If easier, I'm also happy to send a longer technical brief in writing for asynchronous review. Whatever fits your team's workflow.

Thank you for the work you do. Whatever the outcome of this exchange, the public verifier and the open Decision Schema specification will remain Apache-2.0 / CC-BY-4.0 in perpetuity and free for any organisation to adopt.

With respect,

Curtis
Attestly · Lisbon, Portugal
curtis@attestly.org · https://attestly.org · github.com/attestly/attestly

---

## Send notes (for Curtis when actually sending)

- **Best initial routing**: send to `info@eyewitness.global` with `partnerships@int-bar.org` BCC.
- **Best alternative**: contact form at https://www.eyewitness.global/contact-us, paste this text in the message body.
- **Followup**: if no response within 3 weeks, a single polite nudge through LinkedIn outreach to the eyeWitness Programme Manager (Wendy Betts has historically been the public-facing lead; verify current role before contact).
- **Do not** send before the Attestly v0.1 demo is publicly viewable at attestly.org and the GitHub repo is public — both prerequisites are now met as of 2026-05-25.
- **Do** include the OTF concept note and STF proposal status in any follow-up, but not in the first message — keep the first contact short and let them ask.
