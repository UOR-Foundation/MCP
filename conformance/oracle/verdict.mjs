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
// `waiting` maps a scenario to the issue and the pull request that are to deliver what
// it needs (`issue`, `pr`: numbers; `pr` is null when no pull request delivers it). It
// never changes a status: a waiting scenario that fails is reported as failing, and
// the required scenarios passing are counted without it. It decides one thing, the
// gate of the job (`matchesWaiting`): the job passes only when the set of failing
// scenarios equals the set named in `waiting`, each entry naming its issue or pull
// request. A failing scenario that is not named (`unexplained`), a named scenario
// that does not fail (`stale`) and an entry that names neither an issue nor a pull
// request (`unnamed`) each fail the job, so a regression cannot hide behind the
// known failures and a delivered scenario cannot stay on the list.

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
    const label = entry === undefined ? 'unexplained' : !Number.isInteger(entry.pr) ? 'no delivering pull request' : `waiting on pull request #${entry.pr}`;
    (waits[label] ??= []).push(scenario);
  }
  const stale = Object.keys(waiting).filter((scenario) => !failing.includes(scenario));
  const unexplained = failing.filter((scenario) => !Object.hasOwn(waiting, scenario));
  const unnamed = Object.keys(waiting).filter((scenario) => !Number.isInteger(waiting[scenario]?.issue) && !Number.isInteger(waiting[scenario]?.pr));
  const valid = failing.length === 0 && unexpected.length === 0;
  const matchesWaiting = unexplained.length === 0 && stale.length === 0 && unnamed.length === 0 && unexpected.length === 0;
  return { rows, diagnostics, failing, skipped, waits, stale, unexplained, unnamed, unexpected, valid, matchesWaiting };
}
