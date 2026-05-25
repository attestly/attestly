# Ready-to-send outreach emails (v2 — review applied)

Copy each block. Subjects marked **Subject:**. Send-routing notes follow each email.

Recommended order:

1. Portugal Space — send today (hard ESA BASS deadline 28 Aug, longest lead time)
2. INESC TEC — Monday
3. Católica CRC — Tuesday
4. IT Aveiro — Wednesday

Stagger so responses don't pile up.

Each email aims for: who I am · one-line project · why you specifically · ask · offer to share more.

---

## 1. Portugal Space — ESA BASS letter of authorisation

**To**: `info@portugalspace.pt`  
**Cc** (optional): named ESA BASS delegate at portugalspace.pt/cooperacao-internacional/esa — verify before sending  
**Subject**: ESA BASS Kick-Start (28 Aug cut-off) — letter of authorisation

---

Caros Senhores,

I'm preparing a submission to the ESA BASS Kick-Start round, cut-off 28 August. My understanding is that a Portugal Space letter of authorisation is a submission prerequisite, and I'd like to start that process now to leave time for any back-and-forth.

The project: my company **Conforme** (https://conforme.info) is a live PT-based SaaS for short-term-rental compliance. The Kick-Start proposal integrates Copernicus Sentinel imagery and complementary EO data into the product, for address verification and grey-market occupancy detection across the c. 47,000 registered RNAL properties. €100k total (€75k ESA / €25k Conforme co-financing).

Could you let me know the current process and expected timeline for the letter, and whether you'd prefer a written technical brief or a meeting first? I can come to your offices in Lisbon, or send the draft Kick-Start proposal directly — whichever fits your workflow.

Com os melhores cumprimentos,

Curtis  
Founder, Conforme · ENI Portugal  
hello@conforme.info · +351 [your phone]  
https://conforme.info

**Send-routing note**: General inbox routes correctly, but Portugal Space publishes its ESA cooperation contacts at portugalspace.pt/cooperacao-internacional/esa — Cc the named BASS delegate if listed.

---

## 2. INESC TEC — IAPMEI Vale Inovação partner inquiry

**To**: head of the Centre for Information and Cyber Security (CTIC) at INESC TEC — find current head via https://www.inesctec.pt/en/centres/ctic and use their direct address  
**Cc** (optional): `comunicacao@inesctec.pt` (general inquiries) as fallback  
**Subject**: Small applied R&D engagement — open-source Rust transparency-log library

---

Hi,

I'm a Portuguese founder working on an open-source Rust library called **Attestly** — it implements a Certificate-Transparency-style public log for AI Act Article 12 audit evidence. Working v0.1 is at github.com/attestly/attestly. I'm reaching out specifically because CTIC's protocol-analysis and applied-cryptography work seems like a strong fit for the scope I have in mind.

The ask: I'm preparing an IAPMEI Vale Inovação Simplificado application (€5k–€25k, deadline 30 November) for an **independent cryptographic review** of Attestly's canonical-hashing approach, Signed Tree Head format, and Decision Schema spec — before I freeze v0.1 publicly. That kind of independent eyes-on-the-spec is exactly what the protocol needs before grant reviewers and ecosystem partners look at it.

Two short questions:

- Is INESC TEC currently on IAPMEI's accredited research-entity list?
- Would CTIC be open to a short call to see if the scope is something one of your researchers would want to take on?

Happy to share the repo, the draft Kick-Start scope, and a short technical brief. I'm based in Lisbon, comfortable coming to Porto.

Best,

Curtis  
hello@attestly.org · https://attestly.org  
github.com/attestly/attestly

---

## 3. Católica-Lisbon CRC — IAPMEI Vale Inovação partner inquiry

**To**: identify a Católica CRC researcher working on internet governance, regulatory compliance, or digital-evidence law via https://www.fch.lisboa.ucp.pt — Cc them directly  
**Cc** (optional): `info@research.lisbon.ucp.pt` (general inquiries) as fallback  
**Subject**: Question about cryptographic-evidence law under eIDAS 2 and the AI Act

---

Dear [researcher name],

Two new EU regulations are coming into force in 2026 that interact in ways I haven't seen analysed yet: the AI Act (Article 12 record-keeping, August) and eIDAS 2 (Article 45l on Qualified Electronic Ledgers, December). I'm writing because Católica CRC's work on internet governance and regulatory compliance seems like the right place in Portugal to think about how those two pieces fit together.

A short context: I'm a Lisbon-based founder, and I'm building an open-source reference implementation (**Attestly**, github.com/attestly/attestly) of the kind of cryptographic ledger that Article 45l recognises — designed to satisfy Article 12's audit-log requirement at the same time.

I'm preparing a small IAPMEI Vale Inovação Simplificado application (€5k–€25k, deadline 30 November) and would like to engage Católica CRC as the accredited research partner on **a short position paper analysing how these two regulations interact in practice**. Case-study-grounded, maximum ten pages, intended to be useful to Portuguese SMEs and to inform Member State implementation guidance.

Two short questions:

- Is Católica CRC on IAPMEI's accredited research-entity list?
- Would this kind of small applied research engagement fit your group's interests?

Glad to send the technical context, the draft application scope, and meet in Lisbon when convenient.

Cumprimentos,

Curtis  
hello@attestly.org · https://attestly.org

---

## 4. IT Aveiro — IAPMEI Vale Inovação partner inquiry

**To**: head of the Security and Cryptography group at IT Aveiro — find via https://www.it.pt and use their direct address  
**Cc** (optional): `secretariado@it.pt` (general secretariat) as fallback  
**Subject**: Independent cryptographic review for an open-source Rust library — IAPMEI Vale Inovação

---

Hi,

Quick context: I've shipped v0.1 of an open-source Rust library called **Attestly** (github.com/attestly/attestly) — a transparency-log-style audit primitive aimed at EU AI Act Article 12. Before I freeze the spec publicly, I'd like an independent crypto review.

The scope: SHA-256 canonical-hashing approach (currently a domain-separated JSON encoding, considering migration to RFC 8785 JCS), Ed25519 signature patterns, Merkle tree and Signed Tree Head format, plus a forward look at the PQC migration story. Not novel cryptography — I'm composing well-understood primitives, and I want to make sure the composition holds up before I take it to wider audiences.

There's a small budget for it: I'm preparing an IAPMEI Vale Inovação Simplificado application (€5k–€25k, deadline 30 November). IT Aveiro's Security and Cryptography group is a natural fit; your work on PQC migration in particular is directly relevant.

Two questions:

- Is IT on IAPMEI's accredited research-entity list?
- Would a short scoping call make sense to see if this is something the group would take on?

Repo, draft Kick-Start scope, and technical note all available to share.

Thanks,

Curtis  
hello@attestly.org · https://attestly.org

---

## Tracking

For each sent email, log: date sent · recipient · expected response window (1–2 weeks for Portugal Space + IT Aveiro, 2–3 weeks for the others) · followup nudge date. I can convert this section into a tracking table once you've started sending.
