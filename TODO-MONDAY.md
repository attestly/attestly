# Attestly submission checklist — week of 2026-05-23

> **Target**: submit STF + EIC Step 1 by **Sat 31 May 2026 morning** (24h ahead of deadline buffer).
> **Today**: Fri 23 May 2026.
> **Days until submission**: 8.
>
> Items marked **[YOU]** require Curtis (account creation, signatures, sends). Items marked **[DONE]** were knocked off by Claude over the weekend. Items marked **[CLAUDE]** are still being worked.

---

## This weekend (Sat 24 — Sun 25 May)

Five blockers + two account creations. ~90 min total.

- [ ] **[YOU]** Register GitHub org `attestly` at github.com/organizations/new (5 min, free tier)
- [ ] **[YOU]** Buy domain `attestly.dev` (5 min, ~€12/year at Porkbun or Cloudflare Registrar)
- [ ] **[YOU]** Reserve PyPI `attestly` namespace — see Saturday pre-flight check in `pre-flight-saturday.md`
- [ ] **[YOU]** Reserve npm `@attestly` scope
- [ ] **[YOU]** Add remote + push: `git remote add origin git@github.com:attestly/attestly.git && git push -u origin main` (local repo is already initialised)
- [ ] **[YOU]** Create Sovereign Tech Fund applicant account at sovereign.tech/programs/fund (10 min)
- [ ] **[YOU]** Create EU Funding & Tenders Portal account (ECAS) at ec.europa.eu/info/funding-tenders (20 min — slow registration, allow buffer)
- [ ] **[YOU]** Mental commit: 60% bandwidth for the next 8 days; Conforme P0 only

## Monday 26 May — outreach starts (30 min)

- [ ] **[YOU]** Sign the Conforme letter of support — see `proposals/conforme-letter.html` (open in browser, "Print to PDF"); Andy signs digital
- [ ] **[YOU]** Send Fraunhofer SIT email — see `proposals/outreach.md` §2; named contact details added by Claude
- [ ] **[YOU]** Send INESC TEC email — see `proposals/outreach.md` §3; HASLab contact details added by Claude

## Tuesday 27 May — civil society + regulator outreach (20 min)

- [ ] **[YOU]** Send AlgorithmWatch email — `proposals/outreach.md` §4
- [ ] **[YOU]** Send APDP-CNPD email in Portuguese — `proposals/outreach.md` §5

## Wednesday 28 May — coalition + channel outreach (15 min)

- [ ] **[YOU]** Send EDRi email — `proposals/outreach.md` §6
- [ ] **[YOU]** LinkedIn DM a PwC or Deloitte EU AI Act lead — `proposals/outreach.md` §7

## Thursday 29 May — standards outreach + reviewer onboarding (90 min)

- [ ] **[YOU]** Post CloudEvents WG GitHub Discussion + email chairs — `proposals/outreach.md` §8
- [ ] **[YOU]** Identify 2 external reviewers in your network; ask them to read the proposals
- [ ] **[YOU]** Send proposals to reviewers with explicit "feedback by Friday EOD" ask
- [ ] **[YOU]** Write 1-page founder CV — template at `proposals/founder-cv-template.md`

## Friday 30 May — apply review feedback (60 min)

- [ ] **[YOU]** Apply reviewer feedback to STF + EIC drafts
- [ ] **[YOU]** Final read-aloud of both proposals
- [ ] **[YOU]** Verify all URLs work (GitHub org, demo GIF on attestly.dev, spec link)
- [ ] **[YOU]** Deploy `attestly.dev` landing page — `landing/index.html` ready to upload to Cloudflare Pages

## Saturday 31 May — SUBMIT (90 min, ideally morning)

- [ ] **[YOU]** Submit Sovereign Tech Fund via sovereign.tech/programs/fund (~45 min, paste section by section)
- [ ] **[YOU]** Submit EIC Accelerator Step 1 via Funding & Tenders Portal (~45 min, upload PDF + admin form)
- [ ] **[YOU]** Screenshot both receipt emails for records

---

## Knocked off by Claude over the weekend ✅ ALL DONE

- [x] **[DONE]** Save this to-do list as `TODO-MONDAY.md` — file in project root
- [x] **[DONE]** Conforme letter of support — `proposals/conforme-letter.html` (open in browser, print to PDF) + `proposals/conforme-letter.md` (paste into Google Docs); both have `{{andy_email}}` + `{{andy_phone}}` placeholders for Andy to fill
- [x] **[DONE]** Research named contacts at Fraunhofer + INESC TEC — `proposals/outreach.md` updated:
  - Fraunhofer SIT redirected to **Fraunhofer IAIS** (Sankt Augustin/Bonn) — the institute actually running EU AI Act work
  - Named contact: **Fabian Malms**, Project Lead Trustworthy AI / Certified AI, IAIS Bonn (LinkedIn link in template)
  - INESC TEC named contact: **Bernardo Portela**, HASLab Director, email `bfportela@fc.up.pt` (verified format from his HASLab profile)
- [x] **[DONE]** Verify PyPI + npm namespace — all clean, see `pre-flight-saturday.md` for verification log + fallback names if any get squatted before Saturday
- [x] **[DONE]** Initialise local git repo — `git init` on `main` branch, initial commit `34bb4a5` + weekend prep commit `41ffe98`. Saturday push is: `git remote add origin git@github.com:attestly/attestly.git && git push -u origin main`
- [x] **[DONE]** EIC Step 1 pitch deck content — `proposals/eic-pitch-deck.md`, 10 slides ready to paste into Google Slides (~30-45min of layout work)
- [x] **[DONE]** Landing page — `landing/index.html` (single self-contained file, 1 KB CSS inline) + `landing/attestly-demo.gif` embedded. Deploy guide in `landing/README.md` (Cloudflare Pages: drag the folder, done in ~5 min once the domain is registered)

**Bonus item also done:**

- [x] **[DONE]** Founder CV template — `proposals/founder-cv-template.md`. Fill the bracketed placeholders with your real career history. Includes reviewer-side guidance per Andy's feedback (don't fabricate, lean into entrepreneurial authenticity, quantified achievements per role).

---

## What's deliberately NOT on the list

- **Lda incorporation** — defer until EIC Step 1 returns GO (6 weeks post-submission). Saves €500 + accountant fees if no GO.
- **ANI Voucher Deep Tech application** — triggers only after EIC Step 1 GO + Lda. Weeks 7-12.
- **Pitch deck visual layout** — Claude produces markdown content; final design is yours (~30 min in Google Slides).
- **NLnet 1 October cycle** — re-evaluate after Conforme NLnet decision lands (mid-July).

## Three risks worth knowing about

1. **PyPI / npm namespace squatting** — verified clean on 2026-05-21; re-verified by Claude this weekend. Fallback name documented in `pre-flight-saturday.md`.
2. **ECAS account creation has been slow** — start Saturday not Friday. Sometimes 24-48h delay.
3. **Reviewer availability for Thursday** — identify reviewers Monday, not Thursday. Lead time matters.

---

*Generated by Claude 22-23 May 2026. Update this file as items complete or scope changes.*
