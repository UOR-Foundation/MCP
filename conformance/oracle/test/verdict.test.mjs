// CF-03 and CF-05: the verdict is read from the per-scenario checks.
import assert from 'node:assert/strict';
import test from 'node:test';
import { verdict } from '../verdict.mjs';

const pass = (id) => ({ id, status: 'SUCCESS' });
const required = ['server-stateless', 'tools-list', 'caching'];
const unscored = ['http-header-validation'];

function run(results) {
  return verdict({ required, unscored, results: new Map(Object.entries(results)) });
}

test('every scenario with a passing check and no failing one passes', () => {
  const out = run({
    'server-stateless': [pass('a'), { id: 'w', status: 'WARNING' }],
    'tools-list': [pass('b')],
    caching: [pass('c')],
    'http-header-validation': [pass('d')],
  });
  assert.equal(out.valid, true);
  assert.deepEqual(out.failing, []);
  assert.deepEqual(out.rows.map((row) => row.kind), ['required', 'required', 'required', 'not scored']);
});

test('a failing check names its scenario, its id and the suite message (CF-05)', () => {
  const out = run({
    'server-stateless': [pass('a')],
    'tools-list': [pass('b'), { id: 'tools-list-order', status: 'FAILURE', errorMessage: 'expected sorted names' }],
    caching: [pass('c')],
    'http-header-validation': [pass('d')],
  });
  assert.equal(out.valid, false);
  assert.deepEqual(out.failing, ['tools-list']);
  assert.deepEqual(out.diagnostics, ['tools-list: tools-list-order: expected sorted names']);
});

test('a scenario that produced no checks is a failure, scored or not (CF-05)', () => {
  const out = run({ 'server-stateless': [pass('a')], 'tools-list': [pass('b')], caching: [pass('c')] });
  assert.equal(out.valid, false);
  assert.deepEqual(out.failing, ['http-header-validation']);
  assert.deepEqual(out.diagnostics, ['http-header-validation: the scenario produced no checks']);
});

test('a scenario whose checks were all skipped is not a pass (CF-05)', () => {
  const out = run({
    'server-stateless': [pass('a')],
    'tools-list': [{ id: 's', status: 'SKIPPED' }, { id: 't', status: 'INFO' }],
    caching: [pass('c')],
    'http-header-validation': [pass('d')],
  });
  assert.equal(out.valid, false);
  assert.deepEqual(out.failing, ['tools-list']);
  assert.match(out.diagnostics[0], /^tools-list: 2 checks, none of them a success \(SKIPPED, INFO\)$/);
});

test('a scenario outside the frozen requirements makes the run invalid', () => {
  const out = run({
    'server-stateless': [pass('a')],
    'tools-list': [pass('b')],
    caching: [pass('c')],
    'http-header-validation': [pass('d')],
    'tasks-lifecycle': [pass('e')],
  });
  assert.equal(out.valid, false);
  assert.deepEqual(out.unexpected, ['tasks-lifecycle']);
});

test('a scenario the suite itself skips is reported, not counted as passing, and fails nothing (CF-03)', () => {
  const out = verdict({
    required,
    unscored,
    suiteSkipped: { 'http-header-validation': { reason: 'the suite skips it' } },
    results: new Map(Object.entries({
      'server-stateless': [pass('a')],
      'tools-list': [pass('b')],
      caching: [pass('c')],
      'http-header-validation': [{ id: 'h', status: 'SKIPPED' }],
    })),
  });
  assert.equal(out.valid, true);
  assert.deepEqual(out.failing, []);
  assert.deepEqual(out.skipped, ['http-header-validation']);
  assert.equal(out.rows[3].status, 'skipped by the suite');
});

test('a scenario named as skipped by the suite is held to the rule once the suite exercises it (CF-03)', () => {
  const out = verdict({
    required,
    unscored,
    suiteSkipped: { 'http-header-validation': { reason: 'the suite skips it' } },
    results: new Map(Object.entries({
      'server-stateless': [pass('a')],
      'tools-list': [pass('b')],
      caching: [pass('c')],
      'http-header-validation': [{ id: 'h', status: 'FAILURE', errorMessage: 'wrong status' }],
    })),
  });
  assert.equal(out.valid, false);
  assert.deepEqual(out.failing, ['http-header-validation']);
  assert.deepEqual(out.skipped, []);
});

test('failing scenarios are classified by the pull request they wait on, and never excused (CF-03)', () => {
  const out = verdict({
    required,
    unscored,
    waiting: { 'tools-list': { pr: 30 }, caching: { pr: null }, 'server-stateless': { pr: 32 } },
    results: new Map(Object.entries({
      'server-stateless': [pass('a')],
      'tools-list': [{ id: 'b', status: 'FAILURE' }],
      caching: [{ id: 'c', status: 'FAILURE' }],
      'http-header-validation': [{ id: 'h', status: 'FAILURE' }],
    })),
  });
  assert.equal(out.valid, false);
  assert.deepEqual(out.waits, {
    'waiting on pull request #30': ['tools-list'],
    'no delivering pull request': ['caching'],
    unexplained: ['http-header-validation'],
  });
  assert.deepEqual(out.stale, ['server-stateless']);
});

test('the checks that are neither successes nor failures are counted by status', () => {
  const out = run({
    'server-stateless': [pass('a'), { id: 'w', status: 'WARNING' }, { id: 'i', status: 'INFO' }, { id: 'x', status: 'WARNING' }],
    'tools-list': [pass('b')],
    caching: [pass('c')],
    'http-header-validation': [pass('d')],
  });
  assert.deepEqual(out.rows[0].other, { WARNING: 2, INFO: 1 });
});
