// Self-test for the generated userscript: decode the embedded base64,
// verify it round-trips to the source markdown, and run the section parser
// to confirm we get the expected number of H2 fields.
//
// Run with: node selftest.js
'use strict';

const fs = require('fs');
const path = require('path');

const src = fs.readFileSync(path.join(__dirname, 'grant-form-helper.user.js'), 'utf8');

// Match the b64 array inside the first funder entry.
const startMarker = 'b64: [';
const idx = src.indexOf(startMarker);
if (idx < 0) {
  console.error('FAIL: could not find b64 array');
  process.exit(1);
}
const endIdx = src.indexOf('].join(', idx);
if (endIdx < 0) {
  console.error('FAIL: could not find b64 array terminator');
  process.exit(1);
}
const arrBody = src.slice(idx + startMarker.length, endIdx);
const chunkRe = /'([A-Za-z0-9+/=]+)'/g;
const chunks = [];
let m;
while ((m = chunkRe.exec(arrBody)) !== null) chunks.push(m[1]);
console.log('chunks found:', chunks.length);
const b64 = chunks.join('');
const decoded = Buffer.from(b64, 'base64').toString('utf8');

const original = fs.readFileSync(
  path.join(__dirname, '..', '..', 'proposals', 'stf-form-answers.md'),
  'utf8',
);

console.log('decoded bytes :', Buffer.byteLength(decoded));
console.log('original bytes:', Buffer.byteLength(original));
console.log('roundtrip      :', decoded === original ? 'PASS' : 'FAIL');

if (decoded !== original) {
  // Show first differing offset for debugging.
  for (let i = 0; i < Math.min(decoded.length, original.length); i++) {
    if (decoded.charCodeAt(i) !== original.charCodeAt(i)) {
      console.error('first diff at byte', i, '/ ctx:', JSON.stringify(decoded.slice(Math.max(0, i - 20), i + 20)));
      break;
    }
  }
  process.exit(1);
}

// Mirror parseSections from the userscript.
function parseSections(md) {
  const lines = md.split(/\r?\n/);
  const out = [];
  let scope = null;
  let current = null;
  for (const line of lines) {
    const h1 = /^#\s+(.+?)\s*$/.exec(line);
    const h2 = /^##\s+(.+?)\s*$/.exec(line);
    if (h1 && !h2) { scope = h1[1].trim(); continue; }
    if (h2) {
      if (current) out.push(current);
      current = { heading: h2[1].trim(), scope, bodyLines: [] };
      continue;
    }
    if (current) current.bodyLines.push(line);
  }
  if (current) out.push(current);
  return out.map((s) => ({ ...s, body: s.bodyLines.join('\n').trim() }));
}

const sections = parseSections(decoded);
console.log('sections parsed:', sections.length);

const expectFirst = 'Link to project website (url)';
const expectLast = 'checkbox receiving notifications';
const headings = sections.map((s) => s.heading);
console.log('first heading:', JSON.stringify(headings[0]));
console.log('last heading :', JSON.stringify(headings[headings.length - 1]));

const ok =
  headings[0] === expectFirst &&
  headings[headings.length - 1] === expectLast &&
  sections.length >= 25;

console.log('parse check    :', ok ? 'PASS' : 'FAIL');
console.log('scopes seen    :', [...new Set(sections.map((s) => s.scope))].join(' | '));

// Print field summary table.
console.log('\nfields:');
for (const s of sections) {
  const wc = (s.body.match(/\S+/g) || []).length;
  console.log(`  [${s.scope || '-'}] ${s.heading} (${wc} words)`);
}

if (!ok) process.exit(1);
