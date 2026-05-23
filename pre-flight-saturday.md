# Pre-flight check — Saturday registration of names

> Verified by Claude 2026-05-23 17:00 Lisbon.

## All three namespaces are CLEAN as of today

| Namespace | Status | How verified | Fallback if taken on Saturday |
|---|---|---|---|
| **GitHub org `attestly`** | Available (404 on `api.github.com/orgs/attestly`) | GitHub API | `attestly-eu`, `attestly-io`, `attestlyhq` |
| **PyPI `attestly`** | Available (404 on `pypi.org/pypi/attestly/json`) | PyPI JSON API | `attestly-sdk`, `attestly-core` |
| **npm `@attestly/sdk`** | Available (404 on `registry.npmjs.org/@attestly%2Fsdk`) | npm registry API | `@attestly-eu/sdk`, `attestly-sdk` (unscoped) |
| **Domain `attestly.dev`** | Not pre-verified (WHOIS lookup not run); high likelihood available — verify at Porkbun before paying | — | `attestly.io` (~$30/yr), `attestly.eu` (~$15/yr if EU verification possible) |

## Order of operations Saturday (recommended)

1. **Buy domain first** (`attestly.dev`) at porkbun.com or cloudflare.com/products/registrar. ~5 min, ~€12/year. If `.dev` is unavailable, fall back to `.io`. Avoid `.com` — likely taken or expensive.

2. **Register GitHub org** at github.com/organizations/new → free tier. Use `attestly`. Add `attestly.dev` to org settings later.

3. **Reserve PyPI namespace** — slightly fiddly. The standard approach:
   ```bash
   # In a fresh dir
   mkdir attestly-placeholder && cd attestly-placeholder
   # Create minimal pyproject.toml claiming the name
   # See packaging.python.org/en/latest/tutorials/packaging-projects/
   python -m build
   python -m twine upload dist/*
   ```
   The simplest path: pip-install a tiny placeholder, push to PyPI under name `attestly`. Then plan v0.1.0 actual release later.

4. **Reserve npm scope** at npmjs.com/signup, then:
   ```bash
   npm init -w @attestly/placeholder -y
   # Publish a minimal placeholder under the scope
   npm publish --access public
   ```

5. **Push to GitHub**:
   ```bash
   cd C:\claude\attestly
   git remote add origin git@github.com:attestly/attestly.git
   # OR https form: git@github.com:attestly/attestly.git
   git push -u origin main
   ```

## If any name is unexpectedly taken

Each fallback above is fine. The proposal text refers to `Attestly` as the project name; the GitHub org / PyPI / npm names are technical details that can be tweaked without changing the proposal.

Worst case: pick a fully new name. Top alternatives (re-verified for availability at name-check time):
- `Attestor` — slightly redundant but defensible
- `Veritrace` — clean but a bit cute
- `Provenant` — taken on GitHub as `provenant-dev`, avoid
- `Auditly` — verify availability fresh

## ECAS account creation (also Saturday)

Allow 30 min. ECAS sometimes shows the account as "pending" for 24-48h before submission rights are active. Start Saturday morning, not Friday night.

Get the account verified before Friday EOD so the submission isn't blocked.
