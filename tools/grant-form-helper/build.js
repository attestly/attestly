#!/usr/bin/env node
/*
 * build.js — assemble grant-form-helper.user.js from the userscript template
 * and the verbatim contents of proposals/<funder>-form-answers.md.
 *
 * Usage: node build.js
 *
 * Re-run this whenever a sourced *-form-answers.md file changes. The base64
 * payload(s) below are regenerated and inlined into grant-form-helper.user.js
 * so the userscript stays self-contained (no @require, no runtime fetch).
 *
 * Apache-2.0 — see ./LICENSE
 */
'use strict';

const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..', '..');
const proposalsDir = path.join(repoRoot, 'proposals');
const outFile = path.join(__dirname, 'grant-form-helper.user.js');
const templateFile = path.join(__dirname, 'template.user.js');

// Funder registry. To add a new funder:
//   1. drop proposals/<id>-form-answers.md in proposals/
//   2. add an entry below with the apply-portal @match
//   3. node build.js
const FUNDERS = [
  {
    id: 'stf',
    label: 'Sovereign Tech Fund',
    sourceFile: 'stf-form-answers.md',
    matches: ['https://apply.sovereigntechfund.de/*'],
  },
  // Future funders (add @match + drop a proposals/<id>-form-answers.md file):
  // { id: 'otf',   label: 'Open Tech Fund',   sourceFile: 'otf-form-answers.md',   matches: ['https://www.opentech.fund/*'] },
  // { id: 'nlnet', label: 'NLnet NGI0',       sourceFile: 'nlnet-form-answers.md', matches: ['https://nlnet.nl/propose/*'] },
];

function chunk(b64) {
  return b64.match(/.{1,76}/g).map((s) => `    '${s}'`).join(',\n');
}

const payloads = [];
const allMatches = new Set();
for (const f of FUNDERS) {
  const srcPath = path.join(proposalsDir, f.sourceFile);
  if (!fs.existsSync(srcPath)) {
    console.warn(`[build] SKIP ${f.id}: ${srcPath} not found`);
    continue;
  }
  const md = fs.readFileSync(srcPath);
  const b64 = md.toString('base64');
  payloads.push({
    id: f.id,
    label: f.label,
    sourceFile: f.sourceFile,
    matches: f.matches,
    b64Chunks: chunk(b64),
    bytes: md.length,
  });
  for (const m of f.matches) allMatches.add(m);
  console.log(`[build] embedded ${f.id} (${md.length} bytes raw / ${b64.length} bytes b64)`);
}

if (payloads.length === 0) {
  console.error('[build] no funders embedded — aborting');
  process.exit(1);
}

const template = fs.readFileSync(templateFile, 'utf8');

const matchLines = Array.from(allMatches).map((m) => `// @match        ${m}`).join('\n');

const funderObjects = payloads
  .map((p) => {
    return `  {
    id: '${p.id}',
    label: ${JSON.stringify(p.label)},
    sourceFile: ${JSON.stringify(p.sourceFile)},
    matches: ${JSON.stringify(p.matches)},
    b64: [
${p.b64Chunks}
    ].join(''),
  }`;
  })
  .join(',\n');

const out = template
  .replace('/* __MATCH_LINES__ */', matchLines)
  .replace('/* __FUNDERS__ */', funderObjects);

fs.writeFileSync(outFile, out);
console.log(`[build] wrote ${outFile} (${out.length} bytes)`);
