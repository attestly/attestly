# Founder CV — Curtis Smith (template)

> 1-page CV format. Fill in the bracketed placeholders. Aimed at STF and
> EIC reviewers who want to see "can this person responsibly execute".
> Per Andy's feedback: keep it grounded — operational reality beats
> inflated corporate language. Reviewers verify references.

---

# Curtis Smith

Lisbon, Portugal · curts152@gmail.com · {{phone}} · github.com/{{handle}} · {{linkedin}}

---

## Summary

Founder and technical lead with 15+ years building and shipping software for
regulated industries. Polyglot engineer (Rust, Python, TypeScript, Solana,
Roblox Luau). Recent focus: EU regulatory technology and AI accountability
infrastructure.

Currently founder of Conforme, a Portugal-based compliance SaaS for EU short-term
rental operators (in production, paying customers in PT and ES), and lead
developer of Attestly, an open-source verification layer for EU AI Act
Article 12 evidence.

---

## Current work

**Conforme** — Founder & CTO · 2025 – present
- Built and shipped end-to-end EU regulatory compliance SaaS targeting short-term
  rental operators in Portugal and Spain.
- {{N}} paying customers; {{€X}} ARR; {{Y}} listings under management.
- Shipped Layer 2 NRUA registration wizard (full backend + frontend), Operator
  Intel competitive-analysis platform, SIBA guest-check-in automation.
- Sole engineer; coordinates with co-founder Andrew Nicolaou on commercial,
  legal, and partnership functions.

**Attestly** — Founder & sole developer · 2026 – present
- Designed and built open-source verification infrastructure for EU AI Act
  Article 12 evidence (Rust core, Python and TypeScript SDKs, standalone
  verifier).
- Working PoC with 20 automated tests; full end-to-end tamper-detection demo.
- Applications in flight to Sovereign Tech Fund and EIC Accelerator Step 1.

---

## Selected prior work

**{{Prior project/company 1}}** — {{Role}} · {{Year–Year}}
- {{One quantified achievement: shipped X to Y users, reduced Z by N%, etc.}}
- {{One scope sentence: stack, team size, customer profile.}}

**{{Prior project/company 2}}** — {{Role}} · {{Year–Year}}
- {{Quantified achievement.}}
- {{Scope sentence.}}

**{{Prior project/company 3}}** — {{Role}} · {{Year–Year}}
- {{Quantified achievement.}}
- {{Scope sentence.}}

---

## Technical depth (relevant to Attestly)

- **Cryptography in production**: Solana program development including signature
  verification, key derivation, on-chain account validation.
- **Multi-agent systems**: orchestration of Claude Code agent teams for parallel
  feature delivery on Conforme; documented patterns and failure modes.
- **EU regulatory technology**: 12+ months implementing EU directives (DL
  128/2014 + Reg (EU) 2024/1028 for short-term rentals) at code-and-spec level.
- **Polyglot delivery**: production code in Rust, Python (FastAPI, SQLAlchemy,
  Celery), TypeScript (React, Vite), SQL (Postgres), Solana / Anchor (Rust).
- **Open-source posture**: Attestly licensed Apache-2.0 (code) + CC-BY-4.0
  (specifications) from day one; no proprietary components in the grant-funded
  scope.

---

## Education

{{Highest education, year, institution. If experience-based rather than
credentialled — per Andy: "don't hide, don't inflate, compensate with
operational achievements" — list relevant continuing-education or
certifications instead, e.g. courses in cryptography, distributed systems, etc.}}

---

## Selected references (available on request)

- Andrew Nicolaou, co-founder Conforme · {{andy_email}}
- {{Conforme paying customer reference, with permission}}
- {{Optional academic or technical reference}}

---

## Public artefacts

- `github.com/attestly/attestly` — Attestly (Rust, Apache-2.0)
- `github.com/{{conforme_repo}}` — Conforme (private commercial)
- `conforme.info` — production deployment

---

*One page. No photographs. No personal information beyond what is necessary for
funder verification. Update twice per year or when a new deployment / customer
ships.*

---

## Reviewer-side guidance for filling this template

**What to keep**:
- The two-section "Current work" pattern (Conforme + Attestly) — this is your
  strongest operational signal.
- Quantified achievements (customers, ARR, listings under management). Even
  modest numbers beat unspecified claims.
- The technical-depth section linking your existing skills to Attestly's
  primitives. This is the reviewer-confidence-architecture move.

**What to be careful about**:
- Don't invent corporate titles. "Founder & CTO" of Conforme is true; "Senior
  Engineering Manager at $BigCorp" without source would be visible to a
  reference check.
- Don't claim degrees you don't hold. EU grant reviewers verify.
- If you have a thin formal education section, fill the space with concrete
  shipped projects rather than padded coursework.

**What Andy specifically flagged**:
- Lean into entrepreneur/operator authenticity. Don't perform classical
  enterprise executive. Most grant approvals are about "can this person
  responsibly execute" — not "do they have the polished bio of someone who
  has already executed elsewhere".
