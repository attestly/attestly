# attestly.dev — landing page

Self-contained, single-file static landing page. Deploys to any static host.

## Files

- `index.html` — the landing page (single file, all CSS inline)
- `attestly-demo.gif` — embedded demo (~1 MB, copy of `demo-out/attestly-demo.gif`)
- This `README.md`

## Deploy to Cloudflare Pages (~5 min, free tier)

1. Domain registered at Cloudflare (do this Saturday — see `pre-flight-saturday.md`).
2. Cloudflare dashboard → Workers & Pages → Create application → Pages → "Upload assets" (no Git connection needed for the first deploy).
3. Drag the entire `landing/` folder into the upload box. Cloudflare assigns a random `*.pages.dev` subdomain.
4. In project settings → Custom domains → add `attestly.dev` and `www.attestly.dev`. Cloudflare auto-configures the DNS since the domain is registered with them.
5. HTTPS is automatic.

Total time: ~10 minutes. Cost: €0 (free tier covers ~unlimited static traffic).

## Deploy alternative — GitHub Pages

If you'd rather host on GitHub:

1. Push the `landing/` folder to a branch called `gh-pages` in the `attestly/attestly` repo
2. In the repo's Settings → Pages → enable Pages from `gh-pages` branch, root
3. Add `attestly.dev` as the custom domain
4. Configure DNS at Cloudflare: CNAME `attestly.dev` → `attestly.github.io`

Total time: ~10 minutes. Cost: €0.

## Local preview before deploying

```sh
cd landing
python -m http.server 8080
```

Open `http://localhost:8080`. Inspect on mobile via DevTools responsive view.

## What still needs replacing before going live

- The footer "Specifications" links currently point to `#` — wire them to `https://spec.attestly.dev/` once that subdomain is up.
- The footer "Article 12 mapping doc" and "GDPR posture" links point to `#` — wire them to `proposals/regulator-brief.md` (publish a copy to the spec site) once available.
- Consider adding a `<link rel="canonical">` tag and Open Graph meta tags for sharing once the URL stabilises.
- Add Plausible / Fathom analytics if you want post-launch metrics (optional, not required).
