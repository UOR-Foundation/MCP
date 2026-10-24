#!/usr/bin/env node
// Server-conformance oracle for MCP 2026-07-28, run against a served core.
//
// Starts the subject (the generated host program, `serve`), puts a recording relay
// in front of it, runs the pinned conformance suite's `server --requirements
// 2026-07-28` runner through the relay, and decides from the suite's per-scenario
// check files, not from its exit status: that runner exits 1 for harness errors too.
//
// Adapted from PrismPM#247 `sdk/oracles/mcp-conformance-server.mjs` (the subject
// there is a node server that reads PORT; here it is a native program that prints
// `listening 127.0.0.1:PORT`), with two additions: every scenario of the frozen
// requirements file, scored or not, must pass, and the exchanges are written as a
// transcript for the schema oracle (schema-oracle.mjs).
//
// usage: server-suite.mjs --suite-root DIR --out DIR [--waiting FILE] SUBJECT [SUBJECT_ARG...]
//
// FILE (default: conformance/waiting.json) names the failing scenarios that wait on
// another issue and pull request, and the scenarios the suite itself skips. It never
// turns a failure into a pass: a waiting scenario is reported as failing, the
// required scenarios passing are counted without it, and report.json says
// `valid: false` while any scenario fails. It decides the exit status.
//
// Writes OUT/report.json and OUT/transcript.json.
// Exit 0: the set of failing scenarios equals the set FILE names as waiting, each entry
// naming its issue or pull request (every other scenario of the frozen requirements,
// server leg, ran and passed, or is skipped by the suite). With nothing waiting this is
// the case that every scenario passed.
// Exit 1: a scenario failed that FILE does not name (the diagnostic names each failing
// check as `scenario: check id: message`), or a scenario FILE names did not fail
// (`named as waiting but not failing`), or an entry names neither an issue nor a pull
// request, or the subject cannot be started.
// Exit 2: the oracle cannot run, or the suite's requirements differ from the frozen file.
import { spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { createRequire } from 'node:module';
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { exchanges, startRecorder, transcript } from './recorder.mjs';
import { verdict } from './verdict.mjs';

const REVISION = '2026-07-28';
// The frozen requirements file of the suite commit 232a9014.
const FROZEN_REQUIREMENTS_SHA256 = 'ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57';
const READY_MILLIS = 60000;
const SUITE_MILLIS = 600000;
const TAIL_BYTES = 8192;
const MAX_DIAGNOSTICS = 12;
const LINE_BYTES = 300;

class Unavailable extends Error {}

function usage() {
  process.stderr.write('usage: server-suite.mjs --suite-root DIR --out DIR [--waiting FILE] SUBJECT [SUBJECT_ARG...]\n');
  process.exit(2);
}

let suiteRoot;
let outDirectory;
let waitingFile = join(dirname(fileURLToPath(import.meta.url)), '..', 'waiting.json');
const positional = [];
const args = process.argv.slice(2);
for (let index = 0; index < args.length; index += 1) {
  if (args[index] === '--suite-root') suiteRoot = args[++index];
  else if (args[index] === '--out') outDirectory = args[++index];
  else if (args[index] === '--waiting') waitingFile = resolve(args[++index] ?? '');
  else {
    positional.push(...args.slice(index));
    break;
  }
}
if (!suiteRoot || !outDirectory || positional.length === 0) usage();
suiteRoot = resolve(suiteRoot);
outDirectory = resolve(outDirectory);
const [subjectCommand, ...subjectArguments] = positional;

function collect(stream) {
  const state = { chunks: [], size: 0 };
  stream.on('data', (chunk) => {
    state.chunks.push(chunk);
    state.size += chunk.length;
    while (state.size > TAIL_BYTES * 4 && state.chunks.length > 1) state.size -= state.chunks.shift().length;
  });
  return () => {
    const all = Buffer.concat(state.chunks);
    return all.subarray(Math.max(0, all.length - TAIL_BYTES)).toString('utf8');
  };
}

// The suite owns which scenarios are required; this oracle re-reads the same frozen
// file and refuses to run if it is not the one the pin was made for.
function loadRequirements() {
  const file = join(suiteRoot, 'requirements', `${REVISION}.yaml`);
  let bytes;
  try {
    bytes = readFileSync(file);
  } catch (error) {
    throw new Unavailable(`the frozen requirements file is unreadable: ${error.message}`);
  }
  const digest = createHash('sha256').update(bytes).digest('hex');
  if (digest !== FROZEN_REQUIREMENTS_SHA256) {
    throw new Unavailable(`the suite's ${REVISION} requirements differ from the frozen file: expected ${FROZEN_REQUIREMENTS_SHA256}, observed ${digest}`);
  }
  const { parse } = createRequire(join(suiteRoot, 'package.json'))('yaml');
  const parsed = parse(bytes.toString('utf8'));
  const required = parsed.server ?? [];
  const unscored = (parsed.not_scored ?? []).filter((entry) => entry.leg === 'server').map((entry) => entry.scenario);
  if (required.length === 0 || new Set([...required, ...unscored]).size !== required.length + unscored.length) {
    throw new Unavailable('the frozen requirements do not name distinct server scenarios');
  }
  return { required, unscored };
}

// `label: a, b, c` cut into lines of at most LINE_BYTES, so that a log tail or an
// annotation that cuts long lines still names every scenario.
function wrapped(label, names) {
  const lines = [];
  let line = `${label} (${names.length}):`;
  let fresh = true;
  for (const name of names) {
    if (line.length + name.length + 2 > LINE_BYTES) {
      lines.push(line);
      line = ' ';
      fresh = true;
    }
    line += `${fresh ? ' ' : ', '}${name}`;
    fresh = false;
  }
  lines.push(line);
  return lines;
}

function loadWaiting() {
  let parsed;
  try {
    parsed = JSON.parse(readFileSync(waitingFile, 'utf8'));
  } catch (error) {
    throw new Unavailable(`the waiting file ${waitingFile} is unreadable: ${error.message}`);
  }
  return { waiting: parsed.waiting ?? {}, suiteSkipped: parsed.suite_skipped ?? {} };
}

async function startSubject(runDirectory) {
  const child = spawn(subjectCommand, subjectArguments, {
    cwd: runDirectory,
    env: { PATH: process.env.PATH ?? '/usr/bin:/bin', HOME: runDirectory, TMPDIR: runDirectory },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  const stderr = collect(child.stderr);
  let exited = null;
  let announced = null;
  let pending = '';
  child.stdout.on('data', (chunk) => {
    pending += chunk.toString('utf8');
    const match = /^listening 127\.0\.0\.1:(\d+)$/m.exec(pending);
    if (match && announced === null) announced = Number(match[1]);
  });
  child.once('exit', (code, signal) => { exited = { code, signal }; });
  child.once('error', (error) => { exited = { code: null, signal: error.message }; });
  const deadline = Date.now() + READY_MILLIS;
  while (Date.now() < deadline) {
    if (exited) throw new Error(`the server exited before listening (code ${exited.code}, signal ${exited.signal}): ${stderr()}`);
    if (announced !== null) return { child, port: announced, stderr };
    await new Promise((done) => setTimeout(done, 50));
  }
  child.kill('SIGKILL');
  throw new Error(`the server did not print "listening 127.0.0.1:PORT" within ${READY_MILLIS} ms: ${pending}${stderr()}`);
}

function stop(child) {
  return new Promise((done) => {
    if (child.exitCode !== null || child.signalCode !== null) return done();
    child.once('exit', () => done());
    child.kill('SIGTERM');
    setTimeout(() => child.kill('SIGKILL'), 2000).unref();
  });
}

function runSuite(url, resultsDirectory) {
  return new Promise((resolveSuite, reject) => {
    const child = spawn(process.execPath, [join(suiteRoot, 'dist', 'index.js'), 'server', '--url', url, '--requirements', REVISION, '-o', resultsDirectory], {
      cwd: suiteRoot,
      env: { PATH: process.env.PATH ?? '/usr/bin:/bin', HOME: resultsDirectory, TMPDIR: resultsDirectory },
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const stdout = collect(child.stdout);
    const stderr = collect(child.stderr);
    const timer = setTimeout(() => child.kill('SIGKILL'), SUITE_MILLIS);
    child.once('error', (error) => { clearTimeout(timer); reject(new Unavailable(`the suite could not start: ${error.message}`)); });
    child.once('exit', (code) => { clearTimeout(timer); resolveSuite({ code, stdout: stdout(), stderr: stderr() }); });
  });
}

// `server-<scenario>-<ISO timestamp>/checks.json`, one directory per scenario run.
function readResults(resultsDirectory) {
  const results = new Map();
  for (const name of readdirSync(resultsDirectory)) {
    const match = /^server-(.+)-\d{4}-\d{2}-\d{2}T[\d-]+Z$/.exec(name);
    if (!match) continue;
    const file = join(resultsDirectory, name, 'checks.json');
    if (!existsSync(file)) continue;
    const checks = JSON.parse(readFileSync(file, 'utf8'));
    if (!Array.isArray(checks)) throw new Unavailable(`${name}/checks.json is not a list`);
    results.set(match[1], (results.get(match[1]) ?? []).concat(checks));
  }
  return results;
}

async function main() {
  const { required, unscored } = loadRequirements();
  const { waiting, suiteSkipped } = loadWaiting();
  if (!existsSync(join(suiteRoot, 'dist', 'index.js'))) throw new Unavailable('the built conformance suite is absent');
  mkdirSync(outDirectory, { recursive: true });

  const work = mkdtempSync(join(process.env.TMPDIR || tmpdir(), 'mcp-conformance-'));
  const runDirectory = join(work, 'subject');
  const resultsDirectory = join(work, 'results');
  mkdirSync(runDirectory);
  mkdirSync(resultsDirectory);
  let subject;
  let relay;
  try {
    subject = await startSubject(runDirectory);
    relay = await startRecorder(subject.port);
    const suite = await runSuite(`http://127.0.0.1:${relay.port}/mcp`, resultsDirectory);
    const results = readResults(resultsDirectory);
    const record = exchanges(relay.connections);
    writeFileSync(join(outDirectory, 'transcript.json'), `${JSON.stringify(transcript(record))}\n`);

    const { rows, diagnostics, failing, skipped, waits, stale, unexplained, unnamed, unexpected, valid, matchesWaiting } = verdict({ required, unscored, results, suiteSkipped, waiting });
    const report = {
      oracle: `mcp-conformance/232a9014+${REVISION}`,
      requirements_sha256: FROZEN_REQUIREMENTS_SHA256,
      required: { scenarios: required.length, passing: rows.filter((row) => row.kind === 'required' && row.status === 'pass').length },
      not_scored: { scenarios: unscored.length, passing: rows.filter((row) => row.kind === 'not scored' && row.status === 'pass').length },
      failing,
      waits,
      skipped_by_the_suite: skipped,
      stale_waiting: stale,
      unexplained,
      unnamed_waiting: unnamed,
      unexpected,
      suite_exit: suite.code,
      exchanges: record.length,
      scenarios: rows,
      valid,
      matches_waiting: matchesWaiting,
    };
    writeFileSync(join(outDirectory, 'report.json'), `${JSON.stringify(report, null, 2)}\n`);
    process.stdout.write(`${JSON.stringify({ ...report, scenarios: undefined })}\n`);

    if (failing.length > 0 || stale.length > 0 || unnamed.length > 0) {
      // The failures nobody is waiting for come first: they are the ones that fail the job.
      if (unexplained.length > 0) process.stderr.write(`${wrapped('FAILING and not named in waiting.json', unexplained).join('\n')}\n`);
      const ordered = [...diagnostics].sort((a, b) => Number(unexplained.includes(b.split(': ')[0])) - Number(unexplained.includes(a.split(': ')[0])));
      for (const line of ordered.slice(0, MAX_DIAGNOSTICS)) process.stderr.write(`${line}\n`);
      if (ordered.length > MAX_DIAGNOSTICS) process.stderr.write(`... ${ordered.length - MAX_DIAGNOSTICS} further failing checks\n`);
      process.stderr.write(`${failing.length} failing scenarios, by what they wait on:\n`);
      for (const [label, scenarios] of Object.entries(waits)) {
        for (const line of wrapped(label, scenarios)) process.stderr.write(`${line}\n`);
      }
      if (skipped.length > 0) process.stderr.write(`${wrapped('skipped by the suite', skipped).join('\n')}\n`);
      if (stale.length > 0) process.stderr.write(`${wrapped('named as waiting but not failing', stale).join('\n')}\n`);
      if (unnamed.length > 0) process.stderr.write(`${wrapped('named as waiting without an issue or a pull request', unnamed).join('\n')}\n`);
      if (!matchesWaiting) {
        process.stderr.write(`the failing scenarios are not the ones ${waitingFile} names\n`);
        return 1;
      }
      // The failures are exactly the listed ones: not a pass of those scenarios, but
      // the state this pull request is in until the pull requests they wait on land.
      process.stderr.write('the failing scenarios are exactly the ones waiting.json names; they are failures and are counted as such\n');
    }
    if (unexpected.length > 0) throw new Unavailable(`the suite ran scenarios outside the frozen requirements: ${unexpected.join(', ')}`);
    // Nothing failed, so a non-zero exit is the runner disagreeing with its own check files.
    if (failing.length === 0 && suite.code !== 0) throw new Unavailable(`the suite exited ${suite.code} with every scenario passing: ${suite.stderr}`);
    return 0;
  } finally {
    if (relay) await relay.close();
    if (subject) await stop(subject.child);
    rmSync(work, { recursive: true, force: true });
  }
}

try {
  process.exitCode = await main();
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exit(error instanceof Unavailable ? 2 : 1);
}
