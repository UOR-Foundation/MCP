// Validates the answer to every vector against the wire schema of the pinned
// conformance suite (src/validation/wire-schema.ts). By hand, not in CI: it needs node
// and a checkout of the suite at commit 232a9014457eaf4c59559916e3d616d2f7f28f05 with its
// dependencies installed (npm ci).
//
//   cargo run -q -p lifecycle-oracle --example dump_responses > responses.json
//   cp crates/lifecycle-oracle/scripts/wire_check.ts <suite>/ && cp responses.json <suite>/..
//   (cd <suite> && npx tsx wire_check.ts)
import { readFileSync } from 'node:fs';
import { wireSchemaErrors } from './src/validation/wire-schema';

const rows = JSON.parse(readFileSync('../responses.json', 'utf8'));
let bad = 0;
for (const r of rows) {
  const errors = wireSchemaErrors('2026-07-28' as any, r.body, r.request.method);
  if (errors.length) bad++;
  console.log(r.id, r.status, r.request.method, errors.length ? errors : 'ok');
}
console.log('answers with violations:', bad, 'of', rows.length);
