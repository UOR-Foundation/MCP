# Helpers for .github/workflows/lexlean.yml. Sourced by its steps, never run.
# Every step runs its command through `gate`, so a failure always leaves its
# last lines where `gh api repos/<owner>/<repo>/check-runs/<id>/annotations`
# can read them: the job log itself cannot be downloaded from every sandbox.

# Lines of output kept in a failure annotation.
GATE_TAIL_LINES=${GATE_TAIL_LINES:-40}

# Escape text for the value of an `::error::` workflow command.
_annotation_data() {
  local text=$1
  text=${text//'%'/'%25'}
  text=${text//$'\r'/'%0D'}
  text=${text//$'\n'/'%0A'}
  printf '%s' "$text"
}

# Escape text for a property (`title=`) of a workflow command.
_annotation_property() {
  local text
  text=$(_annotation_data "$1")
  text=${text//':'/'%3A'}
  text=${text//','/'%2C'}
  printf '%s' "$text"
}

# annotate LEVEL TITLE LINES LOGFILE: one annotation holding the last LINES
# lines of LOGFILE, without terminal colour, each line cut to 400 characters.
annotate() {
  local level=$1 title=$2 lines=$3 log=$4 tail_text
  tail_text=$(tail -n "$lines" "$log" |
    sed -e 's/\x1b\[[0-9;]*[A-Za-z]//g' | cut -c1-400)
  printf '::%s title=%s::%s\n' "$level" \
    "$(_annotation_property "$title")" \
    "$(_annotation_data "$tail_text")"
}

# gate LABEL COMMAND...: run COMMAND and show its output. When it fails, emit an
# error annotation of its last $GATE_TAIL_LINES lines and return its exit status.
# When it passes, emit a notice of its last line and its duration, so a run that
# executed nothing cannot look like a run that passed. COMMAND runs in a
# subshell, so it passes anything to later steps through $GITHUB_PATH or
# $GITHUB_OUTPUT.
gate() {
  local label=$1 log status restore started=$SECONDS
  shift
  log=$(mktemp "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/gate.XXXXXX")
  restore=$(set +o)
  set +e
  "$@" 2>&1 | tee "$log"
  status=${PIPESTATUS[0]}
  eval "$restore"
  if ((status != 0)); then
    annotate error "$label failed (exit $status)" "$GATE_TAIL_LINES" "$log"
  else
    annotate notice "$label passed in $((SECONDS - started))s" 1 "$log"
  fi
  rm -f "$log"
  return "$status"
}

# lexlean_rev: the revision .github/lexlean.rev names, or an error.
lexlean_rev() {
  local file=.github/lexlean.rev matches
  matches=$(grep -c '^rev = ' "$file") || true
  if [[ $matches != 1 ]]; then
    echo "$file must hold exactly one 'rev = ' line, found $matches" >&2
    return 1
  fi
  local rev
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$file")
  if [[ -z $rev ]]; then
    echo "$file: rev must be a 40-digit lowercase commit hash" >&2
    return 1
  fi
  printf '%s\n' "$rev"
}

# The tool roots the workflow caches.
LEXLEAN_ROOT=${LEXLEAN_ROOT:-$HOME/.lexlean-bin}

# install_lexlean: build `lexlean` at the pinned revision into $LEXLEAN_ROOT,
# unless that root already holds exactly that revision.
install_lexlean() {
  local rev
  rev=$(lexlean_rev)
  if ! lexlean_installed_at "$rev"; then
    rm -rf "$LEXLEAN_ROOT"
    cargo install --git https://github.com/UOR-Foundation/LexLean \
      --rev "$rev" --locked --root "$LEXLEAN_ROOT" lexlean
  fi
  lexlean_installed_at "$rev"
  echo "$LEXLEAN_ROOT/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$LEXLEAN_ROOT/bin/lexlean" --version
}

# lexlean_installed_at REV: cargo's own record says $LEXLEAN_ROOT was built from
# exactly REV, so a cache that was restored for another key cannot pass.
lexlean_installed_at() {
  grep -qF "UOR-Foundation/LexLean?rev=$1#$1)" "$LEXLEAN_ROOT/.crates.toml" 2>/dev/null &&
    test -x "$LEXLEAN_ROOT/bin/lexlean"
}

# install_elan: the pinned elan release (ELAN_VERSION, ELAN_SHA256), checked
# against its SHA-256 before it is unpacked or run.
install_elan() {
  local dir
  dir=$(mktemp -d)
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
    --output "$dir/elan.tar.gz" \
    "https://github.com/leanprover/elan/releases/download/${ELAN_VERSION}/elan-x86_64-unknown-linux-gnu.tar.gz"
  echo "${ELAN_SHA256}  $dir/elan.tar.gz" | sha256sum --check --strict -
  tar -xzf "$dir/elan.tar.gz" -C "$dir"
  "$dir/elan-init" -y --no-modify-path --default-toolchain none
  echo "$HOME/.elan/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$HOME/.elan/bin/elan" --version
}

# install_lean: the toolchain `lean-toolchain` names, then proof that the
# `lean` elan resolves in this directory is that version.
install_lean() {
  local toolchain version
  toolchain=$(<lean-toolchain)
  version=${toolchain##*:v}
  "$HOME/.elan/bin/elan" toolchain install "$toolchain"
  "$HOME/.elan/bin/lean" --version | tee /dev/stderr | grep -qF "version $version,"
  "$HOME/.elan/bin/lake" --version
}

# install_cargo_deny: the pinned cargo-deny release (CARGO_DENY_VERSION,
# CARGO_DENY_SHA256), checked against its SHA-256 before it is unpacked.
install_cargo_deny() {
  local dir name
  dir=$(mktemp -d)
  name=cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
    --output "$dir/deny.tar.gz" \
    "https://github.com/EmbarkStudios/cargo-deny/releases/download/${CARGO_DENY_VERSION}/${name}.tar.gz"
  echo "${CARGO_DENY_SHA256}  $dir/deny.tar.gz" | sha256sum --check --strict -
  tar -xzf "$dir/deny.tar.gz" -C "$dir"
  install -D -m 0755 "$dir/$name/cargo-deny" "$HOME/.cargo/bin/cargo-deny"
  cargo deny --version
}

# install_rust: the toolchain rust-toolchain.toml pins, and nothing else.
install_rust() {
  rustup toolchain install
  rustup show active-toolchain
  rustc --version
  cargo --version
}

# output_pin: publish the pinned revision as the step output `rev`, the key of
# the cache that holds the compiler built from it.
output_pin() {
  local rev
  rev=$(lexlean_rev)
  echo "rev=$rev" >>"${GITHUB_OUTPUT:?}"
}

# output_matrix: publish the jobs that need `lexlean` as the step output
# `matrix`: one that runs the project gates, and one for every production
# request the repository commits, found here and never listed twice.
output_matrix() {
  local found matrix
  found=$(find crates -type f -path '*/generated/request.json' | LC_ALL=C sort)
  if [[ -z $found ]]; then
    echo "no crates/*/generated/request.json: nothing binds a package to its sources" >&2
    return 1
  fi
  matrix=$(printf '%s\n' "$found" | jq -R . | jq -sc '
    {include: ([{kind: "verify", label: "verify", request: ""}] +
      map({kind: "export", label: ("export " + (split("/")[1])), request: .}))}')
  echo "$matrix"
  echo "matrix=$matrix" >>"${GITHUB_OUTPUT:?}"
}

# compiler_semantics_agrees: the language-1.2 compiler-semantics ID of the
# binary built from .github/lexlean.rev is the one lexlean.lock records.
# `lexlean --version` prints the language-1.0 ID, which a language-1.2 lock
# does not record, so the binary is asked the question the lock asks: it locks
# a copy of the committed tree, and the copy's ID is compared with the original.
compiler_semantics_agrees() {
  local scratch recorded built language
  scratch=$(mktemp -d)
  git ls-files -z | xargs -0 cp --parents -t "$scratch"
  language=$(sed -n 's/^language = "\(.*\)"$/\1/p' lexlean.lock)
  recorded=$(sed -n 's/^compiler_semantics = "\(.*\)"$/\1/p' lexlean.lock)
  (cd "$scratch" && lexlean --color never lock >/dev/null)
  built=$(sed -n 's/^compiler_semantics = "\(.*\)"$/\1/p' "$scratch/lexlean.lock")
  rm -rf "$scratch"
  echo "language $language"
  echo "lexlean.lock records compiler-semantics $recorded"
  echo "$(lexlean_rev) computes     compiler-semantics $built"
  if [[ -z $recorded || $recorded != "$built" ]]; then
    echo "The LexLean revision in .github/lexlean.rev is not the compiler that made lexlean.lock." >&2
    echo "Move the revision and the lock together: build that revision and run 'lexlean lock'." >&2
    return 1
  fi
}

# export_matches REQUEST: export REQUEST with `lexlean production build` and
# require that the packages it publishes equal, byte for byte, the packages
# committed beside it, in both directions.
export_matches() {
  local request=$1 dir exported committed name status=0
  dir=$(dirname "$request")
  rm -rf .lexlean/production
  lexlean --color never production build --request "$request"
  exported=$(find .lexlean/production -mindepth 4 -maxdepth 4 -type d \
    -path '*/packages/rust-std/*' | LC_ALL=C sort)
  if [[ -z $exported ]]; then
    echo "$request: the build published no rust-std package" >&2
    return 1
  fi
  committed=$(find "$dir" -mindepth 1 -maxdepth 1 -type d | LC_ALL=C sort)
  # The same packages, by name: neither a committed package the request no
  # longer exports nor an export nobody committed escapes the comparison.
  if [[ $(xargs -r -n1 basename <<<"$exported") != "$(xargs -r -n1 basename <<<"$committed")" ]]; then
    echo "$request: the export and $dir hold different packages" >&2
    echo "exported:  $(xargs -r -n1 basename <<<"$exported" | paste -sd' ')" >&2
    echo "committed: $(xargs -r -n1 basename <<<"$committed" | paste -sd' ')" >&2
    return 1
  fi
  while IFS= read -r package; do
    name=$(basename "$package")
    if ! diff -rq "$package" "$dir/$name" >/dev/null; then
      status=1
      echo "== $dir/$name differs from the export of $request =="
      diff -ru "$dir/$name" "$package" | cut -c1-200 | head -n 20 || true
      echo "== differing files =="
      diff -rq "$dir/$name" "$package" | sed -e "s#^#$request: #" || true
    else
      echo "$dir/$name equals the export of $request"
    fi
  done <<<"$exported"
  return "$status"
}

# prismpm_rev: the revision .github/prismpm.rev names, or an error.
prismpm_rev() {
  local file=.github/prismpm.rev matches rev
  matches=$(grep -c '^rev = ' "$file") || true
  if [[ $matches != 1 ]]; then
    echo "$file must hold exactly one 'rev = ' line, found $matches" >&2
    return 1
  fi
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$file")
  if [[ -z $rev ]]; then
    echo "$file: rev must be a 40-digit lowercase commit hash" >&2
    return 1
  fi
  printf '%s\n' "$rev"
}

# The root the workflow caches the SDK binary in.
PRISMPM_ROOT=${PRISMPM_ROOT:-$HOME/.prismpm-bin}

# output_prismpm_pin: the pinned PrismPM revision as the step output `rev`, the
# key of the cache that holds the binary built from it.
output_prismpm_pin() {
  local rev
  rev=$(prismpm_rev)
  echo "rev=$rev" >>"${GITHUB_OUTPUT:?}"
}

# prismpm_installed_at REV: cargo's own record says $PRISMPM_ROOT was built from
# exactly REV, so a cache restored for another key cannot pass.
prismpm_installed_at() {
  grep -qF "UOR-Foundation/PrismPM?rev=$1#$1)" "$PRISMPM_ROOT/.crates.toml" 2>/dev/null &&
    test -x "$PRISMPM_ROOT/bin/prismpm"
}

# install_prismpm: build `prismpm` at the pinned revision from the PrismPM
# workspace's binary crate, unless $PRISMPM_ROOT already holds exactly that revision.
install_prismpm() {
  local rev
  rev=$(prismpm_rev)
  if ! prismpm_installed_at "$rev"; then
    rm -rf "$PRISMPM_ROOT"
    cargo install --git https://github.com/UOR-Foundation/PrismPM \
      --rev "$rev" --locked --root "$PRISMPM_ROOT" prismpm
  fi
  prismpm_installed_at "$rev"
  echo "$PRISMPM_ROOT/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$PRISMPM_ROOT/bin/prismpm" --version
}

# sdk_copies_agree: the files this repository copies from the PrismPM standard
# library and lexicon, `Foundation.Bytes`, `Foundation.Server.V1.Model` and the
# `prism.arch` lexicon package, are byte for byte the ones at the pinned revision.
sdk_copies_agree() {
  local rev scratch status=0 file
  rev=$(prismpm_rev)
  scratch=$(mktemp -d)
  git -C "$scratch" init -q
  git -C "$scratch" remote add origin https://github.com/UOR-Foundation/PrismPM
  git -C "$scratch" sparse-checkout set stdlib/src/Foundation language/prism.arch
  git -C "$scratch" fetch -q --depth 1 --filter=blob:none origin "$rev"
  git -C "$scratch" checkout -q FETCH_HEAD
  for file in conformance/src/Foundation/Bytes.lex.tex conformance/src/Foundation/Server/V1/Model.lex.tex; do
    if cmp -s "$file" "$scratch/stdlib/src/${file#conformance/src/}"; then
      echo "$file equals the standard library's"
    else
      echo "$file differs from stdlib/src/${file#conformance/src/} at $rev" >&2
      status=1
    fi
  done
  if diff -rq language/prism.arch "$scratch/language/prism.arch"; then
    echo "language/prism.arch equals the SDK's"
  else
    echo "language/prism.arch differs from the SDK's at $rev" >&2
    status=1
  fi
  rm -rf "$scratch"
  return "$status"
}
