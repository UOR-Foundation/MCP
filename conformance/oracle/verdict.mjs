// The verdict of a suite run, decided from the per-scenario checks.
//
// The suite's runner exits 1 for harness errors as well as for failing scenarios and
// exits 0 when a scenario it was told to run produced nothing, so its exit status is
// not the verdict. A scenario of the frozen requirements passes only when it
// produced at least one check, none of them failed and at least one succeeded;
// `no checks` and `no passing check` (every check skipped, or only warnings) are
// failures, and so is every failing check.
//
// One exception is named in a committed file and checked against the run: a scenario
// that the suite itself skips at the pin (`suiteSkipped`: it sends no request and
// reports one SKIPPED check, whatever the server does) is reported as `skipped by
// the suite`. It is not counted as passing; it does not fail the run only while
// every one of its checks is still SKIPPED, so a suite that starts exercising it is
// held to the rule above.
//
// `waiting` maps a scenario to the pull request that is to deliver what it needs
// (`pr`: a number, or null when no pull request delivers it); it only classifies the
// failing scenarios of the report and never changes a status. A scenario named in
// it that does not fail is `stale`.

/**
 * @param {{required: string[], unscored: string[], results: Map<string, object[]>}} input
 *   `results` maps a scenario name to the checks of `checks.json`.
 */
export function verdict({ required, unscored, results, suiteSkipped = {}, waiting = {} }) {
  const rows = [];
  const diagnostics = [];
  for (const [kind, scenarios] of [['required', required], ['not scored', unscored]]) {
    for (const scenario of scenarios) {
      const checks = results.get(scenario) ?? [];
      const failures = checks.filter((check) => check.status === 'FAILURE');
      const passed = checks.filter((check) => check.status === 'SUCCESS').length;
      const skippedBySuite = Object.hasOwn(suiteSkipped, scenario) && checks.length > 0 && checks.every((check) => check.status === 'SKIPPED');
      const status = skippedBySuite ? 'skipped by the suite' : checks.length === 0 ? 'no checks' : failures.length > 0 ? 'fail' : passed === 0 ? 'no passing check' : 'pass';
      const other = {};
      const notes = [];
      for (const check of checks) {
        if (check.status === 'SUCCESS' || check.status === 'FAILURE') continue;
        other[check.status] = (other[check.status] ?? 0) + 1;
        notes.push(`${check.status} ${check.id}: ${check.errorMessage ?? check.description ?? ''}`.trimEnd());
      }
      rows.push({ scenario, kind, status, passed, failed: failures.length, checks: checks.length, other, notes });
      if (checks.length === 0) diagnostics.push(`${scenario}: the scenario produced no checks`);
      for (const check of failures) {
        diagnostics.push(`${scenario}: ${check.id}: ${check.errorMessage ?? check.description ?? 'failed'}`);
      }
      if (checks.length > 0 && failures.length === 0 && passed === 0 && !skippedBySuite) {
        diagnostics.push(`${scenario}: ${checks.length} checks, none of them a success (${[...new Set(checks.map((check) => check.status))].join(', ')})`);
      }
    }
  }
  const unexpected = [...results.keys()].filter((scenario) => !required.includes(scenario) && !unscored.includes(scenario));
  const failing = rows.filter((row) => row.status !== 'pass' && row.status !== 'skipped by the suite').map((row) => row.scenario);
  const skipped = rows.filter((row) => row.status === 'skipped by the suite').map((row) => row.scenario);
  const waits = {};
  for (const scenario of failing) {
    const entry = waiting[scenario];
    const label = entry === undefined ? 'unexplained' : entry.pr === null ? 'no delivering pull request' : `waiting on pull request #${entry.pr}`;
    (waits[label] ??= []).push(scenario);
  }
  const stale = Object.keys(waiting).filter((scenario) => !failing.includes(scenario));
  return { rows, diagnostics, failing, skipped, waits, stale, unexpected, valid: failing.length === 0 && unexpected.length === 0 };
}
