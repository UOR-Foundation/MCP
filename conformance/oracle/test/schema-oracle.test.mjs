// CF-04: the recorded transcripts are validated against the pinned 2026-07-28 schema.
// Needs the installed suite (Ajv) and the vendored schema:
//   MCP_CONFORMANCE_SUITE_ROOT=<installed suite> node --test test/*.test.mjs
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const oracle = join(here, '..', 'schema-oracle.mjs');
const schema = join(here, '..', '..', '..', 'crates', 'lifecycle-oracle', 'suite', 'mcp-spec-80041ab0a139a8f4fb3adbc99f7ae9113ea59671', 'schema', '2026-07-28', 'schema.json');
const suiteRoot = process.env.MCP_CONFORMANCE_SUITE_ROOT;

const META = { 'io.modelcontextprotocol/protocolVersion': '2026-07-28', 'io.modelcontextprotocol/clientCapabilities': {} };
const SERVER = { 'io.modelcontextprotocol/serverInfo': { name: 'mcp-core-fixture', version: '0.1.0' } };
const discover = (id) => ({ jsonrpc: '2.0', id, method: 'server/discover', params: { _meta: META } });
const discovered = (id, extra = {}) => ({
  jsonrpc: '2.0',
  id,
  result: { supportedVersions: ['2026-07-28'], capabilities: {}, ttlMs: 0, cacheScope: 'private', resultType: 'complete', _meta: SERVER, ...extra },
});
const entry = (direction, message, exchange, http = 'POST 200') => ({ direction, message, exchange, http });
const good = () => [entry('client-to-server', discover(1), 0), entry('server-to-client', discovered(1), 0)];

function run(messages, unread = []) {
  assert.ok(suiteRoot, 'MCP_CONFORMANCE_SUITE_ROOT names the installed conformance suite');
  const directory = mkdtempSync(join(tmpdir(), 'schema-oracle-'));
  try {
    const file = join(directory, 'transcript.json');
    writeFileSync(file, JSON.stringify({ version: 'prismpm/mcp-transcript/1', spec: '2026-07-28', messages, unread }));
    const out = spawnSync(process.execPath, [oracle, '--suite-root', suiteRoot, schema, file], { encoding: 'utf8' });
    const text = out.stdout || out.stderr;
    return { status: out.status, report: JSON.parse(text) };
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

test('a valid exchange passes', () => {
  const { status, report } = run(good());
  assert.equal(status, 0);
  assert.equal(report.valid, true);
  assert.equal(report.messages, 2);
});

test('a result without resultType is refused with the schema location (CF-04)', () => {
  const answer = discovered(1);
  delete answer.result.resultType;
  const { status, report } = run([entry('client-to-server', discover(1), 0), entry('server-to-client', answer, 0)]);
  assert.equal(status, 1);
  assert.ok(report.errors.some((error) => error.exchange === 0 && /resultType/.test(error.detail)), JSON.stringify(report));
});

test('a result that is not valid for the method it answers is refused', () => {
  const answer = discovered(1, { supportedVersions: 'latest' });
  const { status, report } = run([entry('client-to-server', discover(1), 0), entry('server-to-client', answer, 0)]);
  assert.equal(status, 1);
  assert.ok(report.errors.length > 0);
});

test('a response that answers no request is refused', () => {
  const { status, report } = run([entry('server-to-client', discovered(7), 3)]);
  assert.equal(status, 1);
  assert.match(report.errors[0].detail, /answers no open client-to-server request/);
});

test('a request the server answered with a result is held to the schema, so an unknown method is refused', () => {
  const request = { jsonrpc: '2.0', id: 5, method: 'tasks/get', params: {} };
  const { status, report } = run([entry('client-to-server', request, 0), entry('server-to-client', { jsonrpc: '2.0', id: 5, result: { resultType: 'complete' } }, 0)]);
  assert.equal(status, 1);
  assert.match(report.errors[0].detail, /not a client request defined by 2026-07-28/);
});

test('a probe the server refused is counted and its error is validated (CF-04)', () => {
  const probe = { jsonrpc: '2.0', id: 5, method: 'server/discover', params: {} };
  const refusal = { jsonrpc: '2.0', id: 5, error: { code: -32602, message: 'Invalid params: _meta is required' } };
  const accepted = run([entry('client-to-server', probe, 0, 'POST 400'), entry('server-to-client', refusal, 0, 'POST 400')]);
  assert.equal(accepted.status, 0);
  assert.equal(accepted.report.refused_probes, 1);
  const malformed = run([entry('client-to-server', probe, 0, 'POST 400'), entry('server-to-client', { jsonrpc: '2.0', id: 5, error: { code: 'bad', message: 'x' } }, 0, 'POST 400')]);
  assert.equal(malformed.status, 1);
});

test('a body that is not JSON is a finding, except the transport refusal of a recorded request (CF-04)', () => {
  const refusal = { exchange: 4, side: 'response', http: 403, reason: 'the body is not JSON (text/plain)' };
  const request = entry('client-to-server', discover(9), 4, 'POST 403');
  const refused = run([...good(), request], [refusal]);
  assert.equal(refused.status, 0);
  assert.equal(refused.report.transport_refusals, 1);
  const other = run([...good(), entry('client-to-server', discover(9), 4, 'POST 500')], [{ ...refusal, http: 500 }]);
  assert.equal(other.status, 1);
  assert.match(other.report.errors[0].detail, /not a JSON-RPC message/);
  const bare = run(good(), [{ exchange: 77, side: 'response', http: 403, reason: 'no request recorded' }]);
  assert.equal(bare.status, 1);
});

test('a transcript of another version, or without messages, is refused', () => {
  assert.equal(run([]).status, 1);
});
