#!/usr/bin/env bash
# Stage the sources the conformance project shares with the main project.
#
# conformance/ is the Prism project root (PrismPM requires the consumer's Cargo workspace
# root to be the project root), and LexLean accepts no path that leaves its project, so the
# modules the conformance root imports are copied beside the project: `src/` of the main
# project and its lexicon `language/mcp.core`, into conformance/shared/, which git ignores.
# The copy is whole and deterministic, so the project's identity is that of the sources it
# is built from. Run it before any `lexlean` or `prismpm` command in conformance/.
#
# usage: stage-shared.sh
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "$here/.." && pwd)
rm -rf "${here:?}/shared"
mkdir -p "$here/shared/language"
cp -r "$root/src" "$here/shared/src"
cp -r "$root/language/mcp.core" "$here/shared/language/mcp.core"
echo "stage-shared: src and language/mcp.core are staged in $here/shared"
