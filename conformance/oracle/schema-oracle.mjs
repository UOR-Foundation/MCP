#!/usr/bin/env node
// Offline schema oracle for recorded MCP 2026-07-28 transcripts.
//
// Every JSON-RPC message in a transcript is validated, with the Ajv that the
// pinned conformance suite locks (ajv 8.20.0 in its package-lock.json), against the
// schema published at the pinned specification commit. Requests and
// notifications are checked against the definition their method names, and a
// response is checked against the definition of the request it answers.
//
// Two kinds of message are accounted for rather than held to the schema, and the
// run reports how many of each it saw:
//   * a refused probe: the suite sends requests that are invalid on purpose (no
//     _meta, an unknown method, a mistyped field) to see them refused. The request is
//     the suite's, not the server's; the server's answer, a JSON-RPC error, is
//     validated like every other response. A request the server answers with a
//     result is never a refused probe and must validate.
//   * a transport refusal: a 403, 406 or 415 whose body is not JSON-RPC is the HTTP
//     refusal of a request before the core sees it (DNS-rebinding protection, the
//     content-type and accept rules). Any other body that is not JSON is a finding.
//
// Adapted from PrismPM#247 `sdk/oracles/mcp-schema-validator.mjs`: Ajv is loaded
// from the installed suite (`--suite-root`), and a transcript may list `unread`
// bodies (not JSON), each of which is a failure here, never a skipped message.
//
// An extension defines messages the specification's schema does not (the tasks extension,
// SEP-2663, keeps its own schema). `--extension SCHEMA --extension-sha256 HEX`, given any
// number of times as pairs, adds an extension schema at a pinned digest: the file must hash
// to the digest or the oracle does not run. A request or notification whose method only an
// extension schema defines is held to that definition, the result of such a request to the
// definition its name gives (`GetTaskRequest` to `GetTaskResult`), and a result whose
// `resultType` is `task` to the extension's `CreateTaskResult`. Without the extension the
// same messages are findings. The run reports how many messages it held to an extension.
//
// One rule of the extension text is applied, and counted in the report: a server SHOULD
// ignore the `inputResponses` of a `tasks/update` that are mapped to a key that is not
// outstanding for the task (tasks.md, "Providing input"). The keys outstanding for a task
// are those of the `inputRequests` of the last `tasks/get` result for it in the transcript;
// the value under any other key is not held to the schema of an input response, and every
// other part of the request still is. The values under the keys that are outstanding are.
//
// usage: schema-oracle.mjs --suite-root DIR [--extension SCHEMA --extension-sha256 HEX]... SCHEMA TRANSCRIPT
//
// Exit 0 when every message is valid, 1 when the transcript is not a valid
// 2026-07-28 exchange (the diagnostic names the message index and the schema
// location), 2 when the oracle itself cannot run.
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { join, resolve } from 'node:path';

const SPEC = '2026-07-28';
const TRANSCRIPT = 'prismpm/mcp-transcript/1';
const MAX_MESSAGES = 100000;
const MAX_ERRORS = 32;
const DIRECTIONS = new Set(['client-to-server', 'server-to-client']);
const TRANSPORT_REFUSALS = new Set([403, 406, 415]);

class Invalid extends Error {}

const fail = (message) => {
  throw new Invalid(message);
};

const USAGE = 'usage: schema-oracle.mjs --suite-root DIR [--extension SCHEMA --extension-sha256 HEX]... SCHEMA TRANSCRIPT\n';
const argv = process.argv.slice(2);
let suiteRoot;
const extensionFiles = [];
const extensionDigests = [];
const positional = [];
for (let index = 0; index < argv.length; index += 1) {
  if (argv[index] === '--suite-root') suiteRoot = argv[++index];
  else if (argv[index] === '--extension') extensionFiles.push(argv[++index]);
  else if (argv[index] === '--extension-sha256') extensionDigests.push(argv[++index]);
  else positional.push(argv[index]);
}
const [schemaPath, inputPath, ...extra] = positional;
if (!suiteRoot || !schemaPath || !inputPath || extra.length > 0 || extensionFiles.some((file) => !file) || extensionFiles.length !== extensionDigests.length || extensionDigests.some((digest) => !/^[0-9a-f]{64}$/.test(digest ?? ''))) {
  process.stderr.write(USAGE);
  process.exit(2);
}
suiteRoot = resolve(suiteRoot);

let validator;
let defs;
// The extensions: for each, its schema, the digest it was read at, and its definitions.
const extensions = [];
try {
  const Ajv2020 = createRequire(join(suiteRoot, 'package.json'))('ajv/dist/2020.js');
  const schema = JSON.parse(await readFile(schemaPath, 'utf8'));
  defs = schema.$defs;
  if (!defs || typeof defs !== 'object' || !defs.JSONRPCMessage) {
    throw new Error('the MCP schema has no JSONRPCMessage definition');
  }
  const ajv = new (Ajv2020.default ?? Ajv2020)({ allErrors: true, strict: false, validateFormats: false });
  ajv.addSchema(schema, 'mcp');
  for (const [index, file] of extensionFiles.entries()) {
    const bytes = await readFile(file);
    const digest = createHash('sha256').update(bytes).digest('hex');
    if (digest !== extensionDigests[index]) {
      throw new Error(`the extension schema ${file} is not the pinned file: sha256 ${digest}, pinned ${extensionDigests[index]}`);
    }
    const extension = JSON.parse(bytes.toString('utf8'));
    if (!extension.$defs || typeof extension.$defs !== 'object') throw new Error(`the extension schema ${file} has no definitions`);
    ajv.addSchema(extension, `ext${index}`);
    extensions.push({ id: extension.$id, sha256: digest, key: `ext${index}`, defs: extension.$defs, held: 0 });
  }
  validator = (name, extension) => {
    const compiled = ajv.getSchema(`${extension ? extension.key : 'mcp'}#/$defs/${name}`);
    if (!compiled) throw new Error(`the ${extension ? extension.id : 'MCP'} schema has no ${name} definition`);
    return compiled;
  };
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exit(2);
}

// Request and notification definitions keyed by the method constant they pin.
function methodTable(unionName) {
  const table = new Map();
  const union = defs[unionName];
  // A union lists its members; a family with one member is that definition itself.
  const members = union?.anyOf ?? (union ? [{ $ref: `#/$defs/${unionName}` }] : []);
  for (const member of members) {
    const name = member.$ref?.split('/').pop();
    const method = defs[name]?.properties?.method?.const;
    if (name && typeof method === 'string') table.set(method, name);
  }
  return table;
}
const clientRequests = methodTable('ClientRequest');
const clientNotifications = methodTable('ClientNotification');
const serverNotifications = methodTable('ServerNotification');

// The method constant a definition pins, directly or in one of the parts of an `allOf`.
function pinnedMethod(definition) {
  const direct = definition?.properties?.method?.const;
  if (typeof direct === 'string') return direct;
  for (const part of definition?.allOf ?? []) {
    const found = part?.properties?.method?.const;
    if (typeof found === 'string') return found;
  }
  return undefined;
}

// The extension definitions keyed by the method they pin: requests (names ending in
// `Request`, their result the same name with `Result`) and notifications.
const extensionRequests = new Map();
const extensionNotifications = new Map();
for (const extension of extensions) {
  for (const [name, definition] of Object.entries(extension.defs)) {
    const method = pinnedMethod(definition);
    if (method === undefined) continue;
    if (name.endsWith('Request') && !clientRequests.has(method)) extensionRequests.set(method, { extension, name });
    else if (name.endsWith('Notification') && !serverNotifications.has(method)) extensionNotifications.set(method, { extension, name });
  }
}
// The keys of `inputRequests` outstanding for each task, as the last `tasks/get` result
// in the transcript showed them, and the request ids that asked for a task.
const outstanding = new Map();
const askedAbout = new Map();
let ignoredInputResponses = 0;
const extensionResultOf = (found) => {
  const result = found.name.replace(/Request$/, 'Result');
  return found.extension.defs[result] ? result : undefined;
};

// Typed JSON-RPC errors are selected by the numeric code each definition pins.
const typedErrors = new Map();
for (const [name, definition] of Object.entries(defs)) {
  const code = definition?.properties?.code?.const;
  if (name.endsWith('Error') && Number.isInteger(code)) typedErrors.set(code, name);
}

// The entries of the transcript being read, so a finding can name its HTTP exchange.
let entries = [];

function check(index, name, value, problems, extension) {
  if (extension) extension.held += 1;
  const validate = validator(name, extension);
  if (validate(value)) return;
  for (const error of validate.errors ?? []) {
    if (problems.length >= MAX_ERRORS) return;
    problems.push({
      message: index,
      exchange: entries[index]?.exchange,
      definition: name,
      location: error.instancePath || '/',
      keyword: error.keyword,
      detail: error.message,
    });
  }
}

try {
  const input = await readFile(inputPath);
  if (input.length === 0) fail('the transcript is empty');
  let transcript;
  try {
    transcript = JSON.parse(input.toString('utf8'));
  } catch (error) {
    fail(`the transcript is not JSON: ${error.message}`);
  }
  if (transcript?.version !== TRANSCRIPT) fail(`the transcript version must be ${TRANSCRIPT}`);
  if (transcript.spec !== SPEC) fail(`the transcript must declare specification ${SPEC}`);
  const messages = transcript.messages;
  if (!Array.isArray(messages) || messages.length === 0 || messages.length > MAX_MESSAGES) {
    fail(`the transcript must hold between 1 and ${MAX_MESSAGES} messages`);
  }

  entries = messages;
  const problems = [];
  let transportRefusals = 0;
  let refusedProbes = 0;
  // A body the recorder could not read as JSON is a finding, not a skipped message,
  // unless it is the transport's refusal of a request that was recorded.
  for (const body of transcript.unread ?? []) {
    if (body.side === 'response' && TRANSPORT_REFUSALS.has(body.http) && messages.some((entry) => entry.exchange === body.exchange && entry.direction === 'client-to-server')) {
      transportRefusals += 1;
    } else if (problems.length < MAX_ERRORS) {
      problems.push({ exchange: body.exchange, definition: null, detail: `the ${body.side} of exchange ${body.exchange} is not a JSON-RPC message: ${body.reason}` });
    }
  }
  const refused = new Set();
  for (const entry of messages) {
    if (entry?.direction === 'server-to-client' && entry.message && 'error' in entry.message) {
      refused.add(`${entry.exchange}:${JSON.stringify(entry.message.id)}`);
    }
  }
  // Open requests by direction, so a response is read against the request it answers.
  const open = { 'client-to-server': new Map(), 'server-to-client': new Map() };

  messages.forEach((entry, index) => {
    // Specific findings name the definition the message was held to; the
    // generic JSON-RPC envelope findings are reported only when none exist.
    const envelope = [];
    const specific = [];
    const note = (text) => specific.push({ message: index, definition: null, detail: text });
    inspect(entry, index, envelope, specific, note);
    for (const problem of specific.length > 0 ? specific : envelope) {
      if (problems.length < MAX_ERRORS) problems.push(problem);
    }
  });

  function inspect(entry, index, envelope, specific, note) {
    const direction = entry?.direction;
    const message = entry?.message;
    if (!DIRECTIONS.has(direction) || message === null || typeof message !== 'object' || Array.isArray(message)) {
      note('an entry needs a direction and a JSON-RPC object');
      return;
    }
    check(index, 'JSONRPCMessage', message, envelope);
    const answering = direction === 'client-to-server' ? 'server-to-client' : 'client-to-server';
    if (typeof message.method === 'string' && 'id' in message) {
      // A request.
      if (direction === 'server-to-client') {
        note(`${SPEC} defines no server-to-client JSON-RPC request; server input is carried in an input-required result`);
        return;
      }
      open[direction].set(JSON.stringify(message.id), message.method);
      if (refused.has(`${entry.exchange}:${JSON.stringify(message.id)}`)) {
        refusedProbes += 1;
        return;
      }
      const definition = clientRequests.get(message.method);
      const defined = extensionRequests.get(message.method);
      if (!definition && defined) {
        let held = message;
        const taskId = message.params?.taskId;
        if (typeof taskId === 'string') askedAbout.set(JSON.stringify(message.id), taskId);
        const responses = message.params?.inputResponses;
        if (message.method === 'tasks/update' && responses !== null && typeof responses === 'object' && !Array.isArray(responses)) {
          const keys = outstanding.get(taskId) ?? new Set();
          const kept = Object.fromEntries(Object.entries(responses).filter(([key]) => keys.has(key)));
          ignoredInputResponses += Object.keys(responses).length - Object.keys(kept).length;
          held = { ...message, params: { ...message.params, inputResponses: kept } };
        }
        check(index, defined.name, held, specific, defined.extension);
        return;
      }
      if (!definition) {
        note(`${message.method} is not a client request defined by ${SPEC}, and the server answered it with a result`);
        return;
      }
      check(index, definition, message, specific);
    } else if (typeof message.method === 'string') {
      // A notification.
      const table = direction === 'client-to-server' ? clientNotifications : serverNotifications;
      const definition = table.get(message.method);
      const defined = direction === 'server-to-client' ? extensionNotifications.get(message.method) : undefined;
      if (!definition && defined) {
        check(index, defined.name, message, specific, defined.extension);
        return;
      }
      if (!definition) {
        note(`${message.method} is not a ${direction} notification defined by ${SPEC}`);
        return;
      }
      check(index, definition, message, specific);
    } else if ('result' in message || 'error' in message) {
      // A response.
      const key = JSON.stringify(message.id);
      const method = open[answering].get(key);
      if (method === undefined && !('error' in message && message.id === undefined)) {
        note(`a response answers no open ${answering} request (id ${key})`);
        return;
      }
      if (method !== undefined) open[answering].delete(key);
      if ('result' in message) {
        const request = clientRequests.get(method);
        const defined = extensionRequests.get(method);
        const created = extensions.find((extension) => extension.defs.CreateTaskResult && message.result?.resultType === 'task');
        const named = request?.replace(/Request$/, 'ResultResponse');
        if (!request && defined) {
          // The envelope is the specification's; the result is the extension's.
          check(index, 'JSONRPCResultResponse', message, specific);
          const result = extensionResultOf(defined);
          if (result) check(index, result, message.result, specific, defined.extension);
          const asked = askedAbout.get(key);
          if (method === 'tasks/get' && asked !== undefined) {
            const requests = message.result?.inputRequests;
            outstanding.set(asked, new Set(requests !== null && typeof requests === 'object' ? Object.keys(requests) : []));
          }
        } else if (created) {
          // A tool call answered with a task: the result is the extension's CreateTaskResult.
          check(index, 'JSONRPCResultResponse', message, specific);
          check(index, 'CreateTaskResult', message.result, specific, created);
        } else {
          check(index, named && defs[named] ? named : 'JSONRPCResultResponse', message, specific);
        }
      } else {
        check(index, 'JSONRPCErrorResponse', message, specific);
        const typed = typedErrors.get(message.error?.code);
        if (typed) check(index, typed, message.error, specific);
      }
    } else {
      note('the object is neither a request, a notification nor a response');
    }
  }

  if (problems.length > 0) {
    process.stderr.write(`${JSON.stringify({ oracle: `mcp-schema/${SPEC}`, valid: false, errors: problems })}\n`);
    process.exit(1);
  }
  process.stdout.write(`${JSON.stringify({ oracle: `ajv/8.20.0+mcp-schema/${SPEC}`, valid: true, messages: messages.length, refused_probes: refusedProbes, transport_refusals: transportRefusals, ignored_input_responses: ignoredInputResponses, extensions: extensions.map((extension) => ({ schema: extension.id, sha256: extension.sha256, checks: extension.held })) })}\n`);
} catch (error) {
  if (error instanceof Invalid) {
    process.stderr.write(`${JSON.stringify({ oracle: `mcp-schema/${SPEC}`, valid: false, errors: [{ detail: error.message }] })}\n`);
    process.exit(1);
  }
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exit(2);
}
