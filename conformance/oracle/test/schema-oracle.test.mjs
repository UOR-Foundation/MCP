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
const tasksSchema = join(here, '..', '..', '..', 'crates', 'lifecycle-oracle', 'suite', 'ext-tasks-899c1bbc869567670093face3591b39c28ce3219', 'schema', '2026-07-28', 'schema.json');
const TASKS_SHA256 = 'bf30afb7ac251e3e22c037b7a685f60ef6603031b5484c0d08b1fa0bbe86d460';
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

function run(messages, unread = [], extension = []) {
  assert.ok(suiteRoot, 'MCP_CONFORMANCE_SUITE_ROOT names the installed conformance suite');
  const directory = mkdtempSync(join(tmpdir(), 'schema-oracle-'));
  try {
    const file = join(directory, 'transcript.json');
    writeFileSync(file, JSON.stringify({ version: 'prismpm/mcp-transcript/1', spec: '2026-07-28', messages, unread }));
    const out = spawnSync(process.execPath, [oracle, '--suite-root', suiteRoot, ...extension, schema, file], { encoding: 'utf8' });
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
  const request = { jsonrpc: '2.0', id: 5, method: 'tools/frob', params: {} };
  const { status, report } = run([entry('client-to-server', request, 0), entry('server-to-client', { jsonrpc: '2.0', id: 5, result: { resultType: 'complete' } }, 0)]);
  assert.equal(status, 1);
  assert.match(report.errors[0].detail, /not a client request defined by 2026-07-28/);
});

// The tasks extension keeps its schema apart from the specification's (TK-05, CF-04): its
// messages are held to it when the oracle is given it, and are findings when it is not.
const WITH_TASKS = ['--extension', tasksSchema, '--extension-sha256', TASKS_SHA256];
const TASK = { taskId: 't-1', status: 'working', createdAt: '2026-10-26T00:00:00.000Z', lastUpdatedAt: '2026-10-26T00:00:00.000Z', ttlMs: 60000, pollIntervalMs: 1000, resultType: 'task', _meta: SERVER };
const call = { jsonrpc: '2.0', id: 3, method: 'tools/call', params: { name: 'slow_compute', arguments: {}, _meta: META } };
const getTask = { jsonrpc: '2.0', id: 4, method: 'tasks/get', params: { taskId: 't-1', _meta: META } };
const tasksExchange = (created, read, readResult) => [
  entry('client-to-server', call, 0),
  entry('server-to-client', { jsonrpc: '2.0', id: 3, result: created }, 0),
  entry('client-to-server', read, 1),
  entry('server-to-client', { jsonrpc: '2.0', id: read.id, result: readResult }, 1),
];

test('the messages of the tasks extension are held to the extension schema (TK-05, CF-04)', () => {
  const read = { ...TASK, resultType: 'complete' };
  const accepted = run(tasksExchange(TASK, getTask, read), [], WITH_TASKS);
  assert.equal(accepted.status, 0, JSON.stringify(accepted.report));
  assert.equal(accepted.report.extensions.length, 1);
  assert.equal(accepted.report.extensions[0].sha256, TASKS_SHA256);
  assert.ok(accepted.report.extensions[0].checks === 3, JSON.stringify(accepted.report));
  // the same exchange without the extension: tasks/get is not a request of the specification
  const bare = run(tasksExchange(TASK, getTask, read));
  assert.equal(bare.status, 1);
  assert.ok(bare.report.errors.some((error) => /tasks\/get is not a client request defined by 2026-07-28/.test(error.detail)), JSON.stringify(bare.report));
});

test('an extension message that breaks the extension schema is a finding (TK-05, CF-04)', () => {
  const wrongStatus = run(tasksExchange({ ...TASK, status: 'sleeping' }, getTask, { ...TASK, resultType: 'complete' }), [], WITH_TASKS);
  assert.equal(wrongStatus.status, 1);
  assert.ok(wrongStatus.report.errors.some((error) => error.definition === 'CreateTaskResult'), JSON.stringify(wrongStatus.report));
  const noId = { ...TASK, resultType: 'complete' };
  delete noId.taskId;
  const missing = run(tasksExchange(TASK, getTask, noId), [], WITH_TASKS);
  assert.equal(missing.status, 1);
  assert.ok(missing.report.errors.some((error) => error.definition === 'GetTaskResult'), JSON.stringify(missing.report));
  const badRequest = run(tasksExchange(TASK, { ...getTask, params: { _meta: META } }, { ...TASK, resultType: 'complete' }), [], WITH_TASKS);
  assert.equal(badRequest.status, 1);
  assert.ok(badRequest.report.errors.some((error) => error.definition === 'GetTaskRequest'), JSON.stringify(badRequest.report));
});

test('a result of a tool call is held to CreateTaskResult only when its resultType says task (TK-05)', () => {
  const plain = { content: [{ type: 'text', text: 'done' }], resultType: 'complete', _meta: SERVER };
  const answered = run([entry('client-to-server', call, 0), entry('server-to-client', { jsonrpc: '2.0', id: 3, result: plain }, 0)], [], WITH_TASKS);
  assert.equal(answered.status, 0, JSON.stringify(answered.report));
  assert.equal(answered.report.extensions[0].checks, 0);
});

test('inputResponses under a key that is not outstanding are ignored, as the extension says a server does (TK-05)', () => {
  const waiting = { ...TASK, status: 'input_required', resultType: 'complete', inputRequests: { name: { method: 'elicitation/create', params: { mode: 'form', message: 'Name?', requestedSchema: { type: 'object', properties: {} } } } } };
  const update = (responses) => ({ jsonrpc: '2.0', id: 5, method: 'tasks/update', params: { taskId: 't-1', inputResponses: responses, _meta: META } });
  const exchange = (responses) => [
    ...tasksExchange(TASK, getTask, waiting),
    entry('client-to-server', update(responses), 2),
    entry('server-to-client', { jsonrpc: '2.0', id: 5, result: { resultType: 'complete' } }, 2),
  ];
  const unknown = run(exchange({ 'unknown-key': { ignored: true } }), [], WITH_TASKS);
  assert.equal(unknown.status, 0, JSON.stringify(unknown.report));
  assert.equal(unknown.report.ignored_input_responses, 1);
  // a value under a key that is outstanding is held to the schema of an input response
  const outstanding = run(exchange({ name: { ignored: true } }), [], WITH_TASKS);
  assert.equal(outstanding.status, 1);
  assert.ok(outstanding.report.errors.some((error) => error.definition === 'UpdateTaskRequest'), JSON.stringify(outstanding.report));
  const answered = run(exchange({ name: { action: 'accept', content: { name: 'A' } } }), [], WITH_TASKS);
  assert.equal(answered.status, 0, JSON.stringify(answered.report));
  assert.equal(answered.report.ignored_input_responses, 0);
  // the rest of the request is still held to the schema
  const noTask = { ...update({ 'unknown-key': {} }), params: { inputResponses: { 'unknown-key': {} }, _meta: META } };
  const bad = run([...tasksExchange(TASK, getTask, waiting), entry('client-to-server', noTask, 2), entry('server-to-client', { jsonrpc: '2.0', id: 5, result: { resultType: 'complete' } }, 2)], [], WITH_TASKS);
  assert.equal(bad.status, 1);
});

test('an extension schema that is not the pinned file does not run the oracle (CF-04)', () => {
  const out = spawnSync(process.execPath, [oracle, '--suite-root', suiteRoot, '--extension', tasksSchema, '--extension-sha256', '0'.repeat(64), schema, '/dev/null'], { encoding: 'utf8' });
  assert.equal(out.status, 2);
  assert.match(out.stderr, /is not the pinned file/);
  const unpaired = spawnSync(process.execPath, [oracle, '--suite-root', suiteRoot, '--extension', tasksSchema, schema, '/dev/null'], { encoding: 'utf8' });
  assert.equal(unpaired.status, 2);
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
