#!/usr/bin/env bash
# Lay the packages `prismpm build` generated beside the binding crate: the core
# package and the two host packages, from the build's `server/` directory into the
# consumer workspace (conformance/Cargo.toml). The compiler's record of the export,
# `production-result.json`, is copied to the second argument when one is given.
#
# usage: lay-out.sh BUILD_DIRECTORY [RESULT_DESTINATION]
#   BUILD_DIRECTORY   .prism/build/<id>, the id `prismpm build` prints
set -euo pipefail

build=${1:?usage: lay-out.sh BUILD_DIRECTORY [RESULT_DESTINATION]}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
server="$build/server"
test -d "$server" || { echo "lay-out: $server is not a server build" >&2; exit 1; }

for package in mcp_conformance_core mcp-conformance-host-api mcp-conformance-host; do
  test -d "$server/$package" || { echo "lay-out: $server/$package is absent" >&2; exit 1; }
  rm -rf "${here:?}/$package"
  cp -r "$server/$package" "$here/$package"
done
if [ -n "${2:-}" ]; then
  cp "$server/production-result.json" "$2"
fi
echo "lay-out: the generated packages of $build are in $here"
