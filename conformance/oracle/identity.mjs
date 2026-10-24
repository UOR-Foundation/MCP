#!/usr/bin/env node
// The identity of the conformance core, the artifact the oracle runs measure.
//
// The core is the package `mcp_conformance_core` that `prismpm build` generates from
// `UorMcp.Fixture.fixtureRun`. Its identity is the digest of its compiled program,
// which does not depend on the compiler that built it, together with the files of the
// package, the sources and lock it was built from and the compiler that built it. This
// reads the compiler's record of the export (`production-result.json`), checks it
// against the package on disk, the lock and the committed export of the same core
// (`prism_mcp`), and writes or checks conformance/core-identity.json, which
// lexlean-mcp checks.
//
// usage: identity.mjs --result production-result.json --lock lexlean.conformance.lock
//          --package DIR --reference prism_mcp/provenance.json (--write FILE | --check FILE)
//
// Exit 0: the record was written, or equals the one computed. Exit 1: the record
// differs, or the inputs disagree with each other (the diagnostic names each
// disagreement). Exit 2: usage or an unreadable input.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const SPEC = 'mcp-conformance/core-identity/1';
const ROOT = 'UorMcp.Fixture.fixtureRun';
const PACKAGE = 'mcp_conformance_core';

export class Disagree extends Error {}

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

/**
 * @param {{result: object, lock: Buffer, packageFiles: Map<string, Buffer>, reference: object}} inputs
 *   `packageFiles` maps a path inside the package directory to its bytes.
 */
export function identity({ result, lock, packageFiles, reference }) {
  const problems = [];
  const expect = (condition, text) => { if (!condition) problems.push(text); };
  const roots = result?.roots ?? [];
  expect(roots.length === 1, `the production result must have exactly one root, found ${roots.length}`);
  const [root] = roots;
  expect(root?.root === ROOT, `the root must be ${ROOT}, found ${root?.root}`);
  const outputs = root?.outputs ?? [];
  expect(outputs.length === 1 && outputs[0]?.target?.id === 'rust-std', 'the root must have exactly one output, for the target rust-std');
  const output = outputs[0];
  expect(output?.package?.name === PACKAGE, `the package must be ${PACKAGE}, found ${output?.package?.name}`);
  expect(output?.certification?.status === 'kernel-checked', `the certification must be kernel-checked, found ${output?.certification?.status}`);
  expect(result?.source?.lock_sha256 === sha256(lock), 'the production result was not made from this lock: lock_sha256 differs from the lock file');
  // The compiler that built the core is the one the lock was made with.
  const recorded = /^compiler_semantics = "([0-9a-f]{64})"$/m.exec(lock.toString('utf8'))?.[1];
  expect(recorded !== undefined && recorded === result?.compiler?.semantics, `the compiler of the production result (${result?.compiler?.semantics}) is not the one the lock records (${recorded})`);

  const prefix = `${output?.package?.directory}/`;
  const files = [];
  for (const entry of output?.package?.files ?? []) {
    const path = entry.path.startsWith(prefix) ? entry.path.slice(prefix.length) : null;
    if (path === null) { problems.push(`the package file ${entry.path} is outside ${prefix}`); continue; }
    const bytes = packageFiles.get(path);
    if (bytes === undefined) { problems.push(`the package file ${path} is absent on disk`); continue; }
    expect(sha256(bytes) === entry.sha256 && bytes.length === entry.byte_length, `the package file ${path} on disk is not the file the production result records`);
    files.push({ path, sha256: entry.sha256, byte_length: entry.byte_length });
  }
  expect(files.length > 0, 'the package lists no files');
  for (const path of packageFiles.keys()) {
    expect(files.some((file) => file.path === path), `the package file ${path} on disk is not in the production result`);
  }

  const program = root?.program?.sha256;
  const provenance = packageFiles.get('provenance.json');
  expect(provenance !== undefined && JSON.parse(provenance.toString('utf8')).program === program, 'the package provenance names another program than the production result');
  expect(reference?.program === program, `the program differs from the committed export ${reference?.name}: ${reference?.program} against ${program}`);

  if (problems.length > 0) throw new Disagree(problems.join('\n'));
  return {
    spec: SPEC,
    root: ROOT,
    program_sha256: program,
    package: { name: PACKAGE, version: output.package.version, export: output.package.export, target: output.target, files },
    source: { source_id: result.source.source_id, semantic_id: result.source.semantic_id, lock_sha256: result.source.lock_sha256 },
    compiler: result.compiler,
    same_program_as: { package: reference.name, program_sha256: reference.program },
  };
}

function read(path) {
  try {
    return readFileSync(path);
  } catch (error) {
    process.stderr.write(`${path} is unreadable: ${error.message}\n`);
    process.exit(2);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const options = new Map();
  const args = process.argv.slice(2);
  for (let index = 0; index < args.length; index += 2) options.set(args[index], args[index + 1]);
  const need = ['--result', '--lock', '--package', '--reference'];
  const mode = options.has('--write') ? '--write' : options.has('--check') ? '--check' : null;
  if (need.some((name) => !options.get(name)) || mode === null || options.size !== need.length + 1) {
    process.stderr.write('usage: identity.mjs --result FILE --lock FILE --package DIR --reference FILE (--write FILE | --check FILE)\n');
    process.exit(2);
  }
  const directory = resolve(options.get('--package'));
  const packageFiles = new Map();
  for (const path of ['Cargo.toml', 'package.json', 'provenance.json', 'src/lib.rs']) packageFiles.set(path, read(join(directory, path)));
  let record;
  try {
    record = identity({
      result: JSON.parse(read(options.get('--result')).toString('utf8')),
      lock: read(options.get('--lock')),
      packageFiles,
      reference: JSON.parse(read(options.get('--reference')).toString('utf8')),
    });
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exit(error instanceof Disagree ? 1 : 2);
  }
  const text = `${JSON.stringify(record, null, 2)}\n`;
  if (mode === '--write') {
    writeFileSync(options.get('--write'), text);
    process.stdout.write(`program ${record.program_sha256}\n`);
  } else if (read(options.get('--check')).toString('utf8') === text) {
    process.stdout.write(`the core is the recorded one: program ${record.program_sha256}\n`);
  } else {
    process.stderr.write(`the core differs from ${options.get('--check')}; computed:\n${text}`);
    process.exit(1);
  }
}
