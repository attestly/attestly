# Attestly — project overview

> Briefing for Andy. Self-contained — no prior context needed.
>
> Prepared 22 May 2026. Curtis.

---

## What Attestly is, in one sentence

**A small open-source library that lets any EU company running a "high-risk AI" system prove to a regulator that its audit trail hasn't been edited after the fact.**

It's the same kind of pattern that Certificate Transparency uses to keep TLS certificates honest, applied to AI decision-making.

---

## Why we're building it

The EU AI Act becomes enforceable on **2 August 2026** — about 10 weeks away. From that date, any company running a "high-risk AI" system (credit scoring, hiring tools, biometric ID, medical diagnostic AI, etc., per Annex III of the Act) is legally required to keep automatic logs of every decision the system makes.

The law says the logs must exist. It is silent on whether the logs can be trusted. Today, a company can edit its own audit trail before a regulator inspects it, and there is no technical way for the regulator to detect the edit. Penalties for record-keeping failures are up to **€15M or 3% of global turnover**.

This is a regulator-side problem, but it is also a vendor problem: every operator of a high-risk AI system now needs to convince its regulator, its customers, and (in disputes) its courts that its audit trail is the genuine record. There is currently no off-the-shelf solution for the bulk of these systems.

**The opportunity is short, real, and time-boxed by a legal deadline.** That combination — legally-required spend + open architectural gap + clear deadline — is what makes it worth investing real time into.

---

## What the software does

Three things, all stitched into one library:

1. **Records every AI decision in a tamper-evident database.** The system signs each decision with a cryptographic key and appends it to a database table that cannot be edited or deleted via normal SQL — only added to.

2. **Periodically publishes a one-line cryptographic fingerprint to a public location.** Not the decisions themselves — just a short signed summary ("here is the Merkle root of all decisions through 14:00 today, signed by us"). Anyone in the world can read this published fingerprint.

3. **Lets a regulator (or affected citizen, or auditor) verify any single decision against the public fingerprint.** They run a command-line tool. It mathematically checks whether the decision the operator is showing them matches the record the operator committed to publicly at the time.

**The trick that makes it GDPR-safe**: only short cryptographic fingerprints get published. The actual decisions, inputs, and personal data stay private in the operator's own database. The public log is meaningless-looking numbers that prove integrity without revealing anything about the people involved.

The closest mental model is **Certificate Transparency**: in 2013 the industry decided that TLS Certificate Authorities had to publish every certificate they issue to a public log so browsers could check for fraud. Within a few years, this stopped being optional. Attestly is the same pattern applied to AI decisions.

---

## Status of the build (as of 22 May 2026)

The project is **further along than the calendar would suggest** because the core is small and the cryptography is well-understood — we're integrating proven primitives, not inventing them.

What is shipped and runnable today:

- **Working Rust library** (3 sub-libraries, ~1,500 lines). Compiles clean, lint-clean, format-clean.
- **20 automated tests passing** — 18 unit tests + 2 end-to-end integration tests. Tests cover identity round-trips, append-only enforcement, Merkle proof correctness, signature verification, the full tamper-detection pipeline, and malformed-bundle handling.
- **End-to-end demo working**. A single command — `bash examples/demo.sh` — runs the full pipeline: spin up a fresh ledger, append 50 synthetic credit decisions, publish a signed checkpoint, export an evidence bundle for one decision, verify it (PASSES), then tamper with the database and verify again (FAILS, with the exact cryptographic mismatch reported).
- **A 27-second animated GIF screencast** of the demo running, produced by a reusable Python renderer. Suitable for embedding in proposals and the project website. File: `demo-out/attestly-demo.gif`.
- **CI matrix builds clean** on Linux, macOS, and Windows.

This took roughly a day of focused engineering, on top of about a week of strategic groundwork (competition checks, sprint planning, grant strategy). Total time spend so far is small.

What is not yet done — but planned for the next 16 weeks if grants come through:

- TypeScript SDK + web-based verifier (so a regulator can drag a bundle into a browser and get a verdict)
- Production-grade public transparency log (using a Google-backed library called Tessera)
- Independent security audit by a respected EU-based firm
- Standards-body engagement (submitting the wire format to CNCF or W3C)

---

## How we got here (and what we deliberately did not build)

Before settling on Attestly, I went through six other candidate ideas and rejected each one after research found an already-shipped competitor or saturated market:

1. AI agent governance toolkit — Microsoft shipped one in April, plus 10 others on GitHub.
2. EUDI Wallet relying-party SDK — the EU itself ships a reference implementation in 7 repos.
3. Post-quantum cryptography migration toolkit — IBM and SandboxAQ already dominate.
4. EU Accessibility scanner — the market chose vendors a year ago.
5. Postgres-native audit log — the closest existing pattern (`pgaudit` + `immudb`) covers it.
6. Fediverse billing engine — addressable market is dozens of operators globally.

Plus a seventh — a CRA conformance toolkit — which looked attractive until I found `craevidence.com` ships exactly that scope as a commercial SaaS and an EU-funded consortium of ~10 open-source projects is shipping the same thing for free this summer.

Attestly survived the same kind of rigorous check because the audit-log integrity layer for *non-agentic* high-risk AI is a real gap that nobody currently owns. The detailed kill-verdict memos are in our notes if you want to read them — bottom line, we did the homework before committing.

---

## The grants we're applying for

Four parallel applications, all non-repayable, all to distinct funders so they don't compete with each other.

| Funder | Amount | What it pays for | Timing | Probability |
|---|---|---|---|---|
| **Sovereign Tech Fund** (Germany / EU-funded) | €100,000 | The open-source library, CLI, web verifier, security audit, standards engagement | Submit by Sat 31 May; decision ~10 weeks; rolling submission so no hard deadline | Moderate. STF specifically funds "open digital base technologies" — Attestly fits their thesis cleanly. Recent awards: curl, OpenSSL, KDE, Postgres infrastructure. |
| **EIC Accelerator Step 1** (European Innovation Council) | €0 directly; **unlocks €60k + €10k ANI Voucher Deep Tech** (Portugal) automatically on a GO | The five-page short application is judged GO/NO-GO; a GO mechanically triggers ANI's voucher, and is eligibility for Step 2 (potentially €1.8M grant + €1.2M optional equity) | Submit same week (next monthly batch); Step 1 decision ~4-6 weeks; no penalty for NO | Low for Step 1 (5-10%), but the cost of trying is small — 5-page form — and the upside is mechanical |
| **NLnet NGI Zero Commons** | €50,000 | Phase 3 follow-on work (federated multi-org logs, EUDI Wallet binding for operator identity). Deferred to October cycle so we don't compete with Conforme's existing NLnet application | Submit 1 October 2026; decision ~6-8 weeks | Moderate. Their thematic fit is excellent. Deferred because we already have one open Conforme application in evaluation with them. |
| Optional follow-on | More | Various PT2030 / Horizon Europe pots if traction shows | 2027 | TBD by then |

**Realistic 12-month non-repayable capture if half of these land**: €150,000 – €250,000.

**Honest probability discussion**: each grant individually is 25-40% probable of landing. The chance that at least one lands is roughly 60-70%. The chance that all four land is small.

The Sovereign Tech Fund + ANI Voucher Deep Tech path is the most reliable single combination — about €160k total if both come through, and they evaluate on completely different criteria so a "yes" on one doesn't influence the other.

---

## What this commits us to

**Time**: ~60% of my working time for roughly 16 weeks if STF approves (mid-July onwards). The other 40% stays on Conforme. Conforme remains the primary commercial product — Attestly is a **side-bet funded by external grants, not by Conforme cashflow**.

**Money**: ~€500 for a Sociedade Unipessoal por Quotas incorporation (only if EIC Step 1 returns GO — required for the ANI Voucher claim). Plus standard accountant fees from then on (~€80/month). If both EIC Step 1 and ANI Voucher fail, the Lda is still useful for Conforme down the line.

**Risk to Conforme**: I have a hard rule built into the project plan — only Conforme P0 incidents interrupt the Attestly sprint. Any P1 or P2 issue gets deferred to post-submission. The 10-day pre-submission window is the only "intense" period; after submission, Attestly drops to a steady-state ~60% time allocation that fits alongside normal Conforme operation.

**Risk to reputation**: deliberately minimal. The library is published under permissive licensing from day 1. If grants don't land, the artifact still exists and can be cited in future applications. The downside scenario is "we did 16 weeks of open-source work and learned a lot but didn't get paid for it" — which is unpleasant but not damaging.

---

## What I'd like from Andy

Two things, neither urgent:

1. **A sanity check on the strategic frame**: does this fit our shared sense of where we're trying to go? Anything I'm missing in the framing for either grant?

2. **If you have any contacts in EU regulatory tech, compliance consulting, Big-4 advisory, or a national AI authority**: a soft introduction once we have the v0.2 polished (~mid-June) would be hugely useful for the "pilot user" claim that both grants want to see substantiated.

There is nothing time-sensitive on your end. The sprint is mostly already plumbed — software works, demo runs, proposals are drafted. The Saturday 31 May submission deadline I'm working toward is self-imposed, not externally locked.

---

## Links + reading order

- **`README.md`** — top-level summary, install + demo instructions
- **`SPRINT-PLAN.md`** — week-by-week plan with grant strategy in detail
- **`CODING-SPRINT.md`** — engineering tasks for the 16-week post-grant build
- **`proposals/stf-proposal.md`** — full Sovereign Tech Fund submission text
- **`proposals/eic-step1.md`** — full EIC Accelerator Step 1 submission text
- **`proposals/README.md`** — non-overlap statement between the two funders
- **`demo-out/attestly-demo.gif`** — the 27-second tamper-detection screencast

If you only want to see one thing, watch the GIF. Forty-seven seconds in, the verifier prints `[FAIL] TAMPERED` in red after the operator tries to alter a decision after the fact. That's the entire thesis of the project.

---

*Happy to talk through any of this on a call. Curtis.*
