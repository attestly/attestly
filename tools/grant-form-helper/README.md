# Grant Form Helper

A browser userscript that turns prepared answers in `proposals/*-form-answers.md` into a sidebar of one-click **Copy** buttons on a grant-application portal. The human still pastes manually into every field.

It is **strictly a paste-helper**:

- It does not fill any form field.
- It does not click any button on the page.
- It does not send any network request, anywhere.
- It does not modify the page outside its own sidebar DOM.
- It does not try to evade any bot defence — there is nothing to evade, because nothing automated happens.

The userscript reads the page (focused-field label, current text length) only to power two small quality-of-life features: highlighting the most likely matching sidebar row when you focus a textarea, and showing a live word-count badge while you type.

---

## What you get

Open a supported grant-application portal (initially `apply.sovereigntechfund.de`) with the userscript installed and a floating sidebar appears on the right edge of the window. The sidebar lists every field from `proposals/stf-form-answers.md` in form order, with:

- Field heading (e.g. *"Describe your project more in-depth (textarea — ~300 words)"*).
- Word count of the prepared answer.
- A **Copy** button that puts the prepared answer on your clipboard.

You then click into the form field in the portal and `Ctrl/Cmd+V` to paste. That's the whole loop.

Other niceties:

- **Drag** the sidebar by its header to move it out of the way.
- **Collapse / expand** with the `–` / `+` button.
- **Filter** field names with the search box at the top.
- **Focus highlight** — click into a form field and the best-matching sidebar row is highlighted (uses field label + placeholder + `aria-label` against a fuzzy match).
- **Live word-count badge** — once a sidebar row is highlighted, its badge switches to *`current / ~target`* and turns red if you go more than 10% over.
- **Multi-funder switcher** — appears at the top of the sidebar once you've embedded more than one funder (see [Adding another funder](#adding-another-funder) below).

---

## Install

1. **Install a userscript manager** in your browser:
   - **Tampermonkey** — Chrome, Edge, Brave, Firefox, Safari ([tampermonkey.net](https://www.tampermonkey.net/))
   - **Violentmonkey** — Chrome, Edge, Firefox ([violentmonkey.github.io](https://violentmonkey.github.io/))
   - **Greasemonkey** — Firefox ([greasemonkey-script-builder.github.io](https://www.greasespot.net/))

2. **Open the raw userscript URL**:

   https://raw.githubusercontent.com/attestly/attestly/main/tools/grant-form-helper/grant-form-helper.user.js

   Your userscript manager will detect the `// ==UserScript==` header and prompt you to install.

3. **Confirm the `@match` URLs** in the install dialog. You should see at least:

   ```
   https://apply.sovereigntechfund.de/*
   ```

   Click **Install**.

4. **Reload the portal**. Visit https://apply.sovereigntechfund.de — the sidebar should appear within a second of the page becoming idle. If it doesn't, see [Troubleshooting](#troubleshooting) below.

---

## Use

1. Sign in to the STF portal and open your application.
2. The sidebar appears on the right edge. Scroll through the field list — it's grouped by tab (*TAB 1 — Entry-form prescreen*, *TAB 2 — Sovereign Tech Fund Proposal*) in the order STF presents them.
3. Click into the first form field on the portal page.
4. Click **Copy** on the matching sidebar row (it should auto-highlight when you focused the form field).
5. Paste with `Ctrl/Cmd+V`.
6. Glance at the live word-count badge to confirm you're not way under or way over the target.
7. Repeat for each field. Save / submit through the portal yourself.

If a sidebar row doesn't match what you expected, use the search box at the top of the sidebar to filter.

---

## Updating the embedded content

The userscript ships the markdown content **inline** as a base64 string, so it is self-contained — no `@require`, no runtime fetch. When you edit `proposals/stf-form-answers.md` (or any other embedded `*-form-answers.md`), regenerate the userscript:

```sh
cd tools/grant-form-helper
node build.js
```

This rewrites `grant-form-helper.user.js` from `template.user.js` plus whatever `proposals/<id>-form-answers.md` files are listed in the `FUNDERS` registry inside `build.js`.

Bump the `@version` line in `template.user.js` when you publish a new revision so installed userscript managers can offer it as an update.

To verify the embedded content matches the source:

```sh
node selftest.js
```

This round-trips the embedded base64 back to UTF-8, compares it byte-for-byte to the source markdown, runs the section parser, and prints the resulting field table.

### Switching to a fetched-at-install model (production)

If you'd rather have the userscript fetch the latest markdown from GitHub at install time instead of embedding it, you can replace the base64 payload with a one-time fetch in `template.user.js`:

```js
// in template.user.js, replace mount() with:
async function mount(funderId) {
  const funder = FUNDERS.find((f) => f.id === funderId) || FUNDERS[0];
  const url = `https://raw.githubusercontent.com/attestly/attestly/main/proposals/${funder.sourceFile}`;
  const md = await fetch(url).then((r) => r.text());
  // ... rest stays the same, except no b64ToUtf8 step
}
```

Trade-offs:

- **Embedded (current)** — works offline, content is frozen at install time, the user must reinstall (or you must bump `@version` and let the userscript manager auto-update) to pick up new answers. No CORS surprises. Userscript file is ~60 KB.
- **Fetched** — always serves the latest markdown without reinstalling. Requires `raw.githubusercontent.com` to be reachable from inside the portal's origin (this is fine in practice — GitHub raw allows cross-origin reads from any origin via `Access-Control-Allow-Origin: *` for public repos). One network request per page load; you may want to switch from `@grant none` to `@grant GM_xmlhttpRequest` if the portal ever ships a CSP that blocks third-party fetches.

---

## Adding another funder

To add a new grant portal (OTF, NLnet, ISF, EIC, EUIPO, etc.):

1. Drop a new file in `proposals/`, e.g. `proposals/otf-form-answers.md`. Use the same structure: `# TAB ...` for scope, `## Field name (type, ~N words)` per field, plain text or a fenced code block for the answer.
2. Open `tools/grant-form-helper/build.js` and add a `FUNDERS` entry:

   ```js
   { id: 'otf', label: 'Open Tech Fund', sourceFile: 'otf-form-answers.md', matches: ['https://www.opentech.fund/apply/*'] },
   ```

3. Run `node build.js`. The generated `grant-form-helper.user.js` will gain a new `// @match` line for the new portal and a second entry in the in-script funder switcher.
4. Re-install (or let Tampermonkey auto-update) and the sidebar will switch funder content automatically based on the active URL.

---

## Anti-bot posture

This script is designed not to trip any anti-bot defence:

- No clicks on portal elements. No keystrokes synthesised. No `dispatchEvent` on any node outside its own sidebar.
- No `MutationObserver`, no DOM polling of form fields, no scraping.
- No network traffic of any kind in the default (embedded) mode.
- No user-agent / fingerprint manipulation. The userscript explicitly does not set `@grant unsafeWindow` and does not touch `window.navigator`.
- The only reads from outside the sidebar are: (a) the focused element's label/placeholder/`aria-label` attributes, (b) the focused element's current value/text — both for cosmetic purposes inside the sidebar.

If you find a portal that flags it anyway, the safe response is to remove that portal from the `@match` list and resume manual copy-paste from the markdown file directly. Do not add evasion code.

---

## Manual smoke test

Against the live portal:

1. Install the userscript.
2. Visit `https://apply.sovereigntechfund.de` and sign in.
3. Open the application form.
4. Confirm the sidebar appears.
5. For each STF field (35 in the current STF markdown), click **Copy** on the matching sidebar row and paste into the portal field. Confirm the pasted text matches `proposals/stf-form-answers.md`.
6. Confirm the live word-count badge updates as you type / paste.
7. Confirm the focus-highlight picks the right sidebar row when you tab between form fields.
8. Confirm dragging the sidebar header repositions it.
9. Confirm collapse/expand works.

Against the offline fixture (recommended for development):

1. Open `tools/grant-form-helper/fixture.html` directly in your browser (e.g. `file:///C:/claude/attestly/tools/grant-form-helper/fixture.html`).
2. In your userscript manager, edit the installed userscript and temporarily add `// @match file:///*` to the header. **Do not commit this change.**
3. Reload the fixture page. Sidebar should appear with the STF fields.
4. Click each Copy button, paste into the corresponding text input, and confirm the content.

---

## Troubleshooting

**Sidebar doesn't appear.**

- Open DevTools → Console and look for `[grant-form-helper]` lines.
- Confirm the userscript is enabled and the URL matches one of the `// @match` patterns. The Tampermonkey toolbar icon lists active scripts for the current tab.
- Some portals serve their app inside an `<iframe>`. This script uses `@noframes` so it only injects in the top window. If the form is inside an iframe and you really want the sidebar there, remove `// @noframes` from the userscript header and reinstall — but be aware that clipboard writes inside iframes are sometimes blocked by the parent page's CSP.

**Copy button shows "Copy failed".**

- Modern browsers require a user gesture before `navigator.clipboard.writeText`. The button click counts, so this should work — but if the portal page is served with a Permissions Policy that restricts `clipboard-write`, the API will reject.
- Workaround: the script falls back to the legacy `document.execCommand('copy')` path automatically. If that also fails, open DevTools and manually select + copy the text from the markdown file in `proposals/stf-form-answers.md`.

**Sidebar conflicts with the portal's layout.**

- Drag it by the header to a less crowded spot.
- Or collapse it (`–` button) and only expand it when needed.

**Tampermonkey doesn't offer to install when you click the raw URL.**

- Older browsers sometimes render `.user.js` as plain text instead of triggering the install dialog. In that case, right-click the link → "Save link as…" → save as `grant-form-helper.user.js`. Then open Tampermonkey dashboard → "Utilities" → "Import from file" or drag-and-drop the file onto the dashboard.

---

## File layout

```
tools/grant-form-helper/
├── grant-form-helper.user.js   # the userscript itself (generated, commit this)
├── template.user.js            # source template — edit this
├── build.js                    # node build.js → regenerates grant-form-helper.user.js
├── selftest.js                 # node selftest.js → verifies embedded markdown round-trips
├── fixture.html                # offline smoke-test page
├── README.md                   # this file
└── LICENSE                     # Apache-2.0
```

---

## License

Apache-2.0. See `LICENSE`.
