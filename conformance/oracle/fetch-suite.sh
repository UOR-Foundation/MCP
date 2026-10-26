#!/usr/bin/env bash
# Install the pinned MCP conformance suite: the exact commit, then `npm ci` from the
# package-lock.json the commit carries (its `prepare` script builds dist/).
#
# usage: fetch-suite.sh DESTINATION   (an absent or empty directory)
#
# The commit is a content hash, so the checkout is the suite the pin names or the
# script fails. The files this repository vendors under
# crates/lifecycle-oracle/suite must equal the checkout's, so the vectors the tests
# take from them are taken from the suite that runs.
set -euo pipefail

COMMIT=cf022cbb541f9d939068d7f84758b9728be7fc23
REPOSITORY=https://github.com/modelcontextprotocol/conformance

dest=${1:?usage: fetch-suite.sh DESTINATION}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
vendored="$here/../../crates/lifecycle-oracle/suite/conformance-$COMMIT"

if [ -e "$dest" ] && [ -n "$(ls -A "$dest")" ]; then
  echo "fetch-suite: $dest is not empty" >&2
  exit 2
fi
mkdir -p "$dest"
git -C "$dest" init -q
git -C "$dest" remote add origin "$REPOSITORY"
git -C "$dest" fetch -q --depth 1 origin "$COMMIT"
git -C "$dest" checkout -q FETCH_HEAD
head=$(git -C "$dest" rev-parse HEAD)
if [ "$head" != "$COMMIT" ]; then
  echo "fetch-suite: fetched $head, pinned $COMMIT" >&2
  exit 1
fi

(cd "$vendored" && find . -type f | sort) | while read -r file; do
  if ! cmp -s "$vendored/$file" "$dest/$file"; then
    echo "fetch-suite: vendored $file differs from the suite at $COMMIT" >&2
    exit 1
  fi
done

(cd "$dest" && npm ci --no-audit --no-fund --loglevel=error)
test -f "$dest/dist/index.js"
echo "fetch-suite: the suite at $COMMIT is installed in $dest"
