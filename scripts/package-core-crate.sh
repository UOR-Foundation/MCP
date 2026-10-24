#!/bin/sh
# The protocol core as a crate: package the committed export of the production root
# UorMcp.Fixture.fixtureRun, and prove that the package is the core the conformance
# build records (MCP#14; PB-01 to PB-05 in model/ids.toml).
#
#   package-core-crate.sh (--release | --dev N | --dev auto) --out DIR [--dry-run]
#       Packages the export, checks it, and writes DIR/<crate>-<version>.crate and
#       DIR/<crate>-<version>.crate.sha256. Nothing is written when a check refuses.
#   package-core-crate.sh (--release | --dev N | --dev auto) --verify FILE.crate
#       Checks an existing archive against the tree.
#
#   --release   the version of the workspace, the version a release tag publishes.
#   --dev N     the pre-release <workspace version>-dev.N, for a commit imported during
#               development. N is the number of first-parent commits up to that commit.
#   --dev auto  the same, with N counted by git in --root (refused in a shallow clone).
#   --dry-run   afterwards run `cargo publish --dry-run` on the same files and require it
#               to verify the same files (needs the network, no credential).
#   --keep DIR  leave the files that were packaged in DIR (which must not exist), outside git,
#               so that the release workflow uploads from them exactly what was checked.
#   --root DIR  the tree to read instead of the checkout this script is in (the tests
#               plant defects in a copy).
#
# Needs sh, cargo, tar, sha256sum, cmp, diff, sed, find and sort. Needs no network (but
# --dry-run), no node and no LexLean. It reads only files of the tree: the export beside
# crates/lifecycle-oracle/generated/request.json, the publication record and README in
# crates/lifecycle-oracle/publication, conformance/core-identity.json, the workspace
# Cargo.toml and the two licence files. It never reads or prints a credential, and it
# never uploads.
#
# The package is built from a copy of the export in a temporary directory outside git,
# so the crate does not depend on the commit it is made at: the same tree gives the same
# crate, and a commit is recoverable from the version, not from the crate.
#
# Every refusal is `refused <code>: <message>` on standard error and exit status 1. The
# codes: input-missing, name-mismatch, version-mismatch, root-mismatch, export-missing,
# export-tampered, program-mismatch, source-mismatch, shallow-clone, publication-record,
# not-reproducible, file-set, file-tampered, manifest-differs, dry-run-differs.
set -eu

export_dir=crates/lifecycle-oracle/generated
publication_dir=crates/lifecycle-oracle/publication
identity_file=conformance/core-identity.json
root_name=UorMcp.Fixture.fixtureRun

here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd -P)
root=$(dirname "$here")

usage() {
    printf 'usage: %s (--release | --dev N | --dev auto) (--out DIR [--dry-run] [--keep DIR] | --verify FILE.crate) [--root DIR]\n' "$0" >&2
    exit 64
}

refuse() {
    printf 'refused %s: %s\n' "$1" "$2" >&2
    exit 1
}

version_flag='' out='' verify='' keep='' dry_run=false
while [ "$#" -gt 0 ]; do
    case $1 in
        --release) version_flag=release ;;
        --dev)
            [ "$#" -ge 2 ] || usage
            version_flag=$2
            shift
            ;;
        --out)
            [ "$#" -ge 2 ] || usage
            out=$2
            shift
            ;;
        --verify)
            [ "$#" -ge 2 ] || usage
            verify=$2
            shift
            ;;
        --root)
            [ "$#" -ge 2 ] || usage
            root=$2
            shift
            ;;
        --dry-run) dry_run=true ;;
        --keep)
            [ "$#" -ge 2 ] || usage
            keep=$2
            shift
            ;;
        *) usage ;;
    esac
    shift
done
[ -n "$version_flag" ] || usage
if [ -n "$out" ] && [ -n "$verify" ]; then usage; fi
if [ -z "$out" ] && [ -z "$verify" ]; then usage; fi
if [ "$dry_run" = true ] && [ -z "$out" ]; then usage; fi
if [ -n "$keep" ] && [ -z "$out" ]; then usage; fi
case $version_flag in
    release | auto) ;;
    *[!0-9]* | '') usage ;;
esac

for tool in cargo tar sha256sum cmp diff sed find sort; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'package-core-crate.sh needs %s\n' "$tool" >&2
        exit 69
    }
done

# The archive's bytes, not only its contents, are the release's when cargo is the version
# rust-toolchain.toml pins: use that toolchain when rustup has it, and say which cargo ran.
channel=$(sed -n 's/^channel = "\(.*\)"$/\1/p' "$root/rust-toolchain.toml" 2>/dev/null | head -n 1)
if [ -z "${RUSTUP_TOOLCHAIN:-}" ] && [ -n "$channel" ] && command -v rustup >/dev/null 2>&1 &&
    rustup toolchain list 2>/dev/null | grep -q "^$channel"; then
    RUSTUP_TOOLCHAIN=$channel
    export RUSTUP_TOOLCHAIN
fi

sha() {
    sha256sum "$1" | cut -d' ' -f1
}

# The value of `KEY = "VALUE"` in a TOML file, the first such line.
toml_value() {
    sed -n "s/^$2 = \"\\(.*\\)\"\$/\\1/p" "$1" | head -n 1
}

# A key of the [workspace.package] table.
workspace_value() {
    sed -n "/^\\[workspace\\.package\\]/,/^\\[/s/^$1 = \"\\(.*\\)\"\$/\\1/p" "$root/Cargo.toml" | head -n 1
}

# provenance.json is one line of sorted keys: these are anchored on its neighbours.
provenance_name() { sed -n 's/.*"name":"\([^"]*\)","profile":.*/\1/p' "$1"; }
provenance_version() { sed -n 's/.*"version":"\([^"]*\)"}.*/\1/p' "$1"; }
provenance_program() { sed -n 's/.*"program":"\([0-9a-f]\{64\}\)".*/\1/p' "$1"; }
provenance_file() { sed -n "s#.*\"$2\":\"\\([0-9a-f]\\{64\\}\\)\".*#\\1#p" "$1"; }

for file in "$export_dir/request.json" "$publication_dir/publication.toml" "$publication_dir/README.md" \
    "$identity_file" Cargo.toml LICENSE-MIT LICENSE-APACHE; do
    [ -f "$root/$file" ] || refuse input-missing "$root/$file is not a file"
done

# 1. One name, one version, one root, in every place that states them.
crate=$(toml_value "$root/$publication_dir/publication.toml" crate)
publication_root=$(toml_value "$root/$publication_dir/publication.toml" root)
description=$(toml_value "$root/$publication_dir/publication.toml" description)
[ -n "$crate" ] && [ -n "$publication_root" ] && [ -n "$description" ] ||
    refuse publication-record "$publication_dir/publication.toml needs crate, root and description"
case $description in
    *\"* | *\\*) refuse publication-record 'the description may hold no quote and no backslash' ;;
esac
release=$(workspace_value version)
license=$(workspace_value license)
repository=$(workspace_value repository)
homepage=$(workspace_value homepage)
[ -n "$release" ] && [ -n "$license" ] && [ -n "$repository" ] && [ -n "$homepage" ] ||
    refuse input-missing 'Cargo.toml [workspace.package] needs version, license, repository and homepage'

identity_root=$(sed -n 's/^  "root": "\(.*\)",$/\1/p' "$root/$identity_file")
[ "$publication_root" = "$root_name" ] && [ "$identity_root" = "$root_name" ] ||
    refuse root-mismatch "the publication record names $publication_root and $identity_file names $identity_root; the core is $root_name"

requested=$(sed -n "s/.*\"package\":{\"name\":\"\\([^\"]*\\)\",\"version\":\"\\([^\"]*\\)\"},\"root\":\"$root_name\".*/\\1 \\2/p" "$root/$export_dir/request.json")
[ -n "$requested" ] || refuse root-mismatch "$export_dir/request.json does not export $root_name"
request_name=${requested% *}
request_version=${requested#* }
[ "$request_name" = "$crate" ] ||
    refuse name-mismatch "the export request names the crate $request_name and the publication record $crate"
[ "$request_version" = "$release" ] ||
    refuse version-mismatch "the export request versions the crate $request_version and the workspace $release"

export_path=$root/$export_dir/$crate
[ -d "$export_path" ] || refuse export-missing "$export_dir/$crate is not a directory"
for file in Cargo.toml package.json provenance.json src/lib.rs; do
    [ -f "$export_path/$file" ] || refuse export-missing "$export_dir/$crate/$file is not a file"
done
[ "$(toml_value "$export_path/Cargo.toml" name)" = "$crate" ] &&
    [ "$(provenance_name "$export_path/provenance.json")" = "$crate" ] ||
    refuse name-mismatch "the manifest and the provenance of the export do not both name $crate"
[ "$(toml_value "$export_path/Cargo.toml" version)" = "$release" ] &&
    [ "$(provenance_version "$export_path/provenance.json")" = "$release" ] ||
    refuse version-mismatch "the manifest and the provenance of the export do not both say $release"
[ "$(provenance_file "$export_path/provenance.json" Cargo.toml)" = "$(sha "$export_path/Cargo.toml")" ] &&
    [ "$(provenance_file "$export_path/provenance.json" src/lib.rs)" = "$(sha "$export_path/src/lib.rs")" ] ||
    refuse export-tampered "Cargo.toml or src/lib.rs of $export_dir/$crate is not the file its provenance.json records: regenerate the export, never edit it"

# 2. The export is the core digest the conformance build records (#13).
program=$(provenance_program "$export_path/provenance.json")
recorded_program=$(sed -n 's/^  "program_sha256": "\([0-9a-f]\{64\}\)",$/\1/p' "$root/$identity_file")
[ -n "$program" ] && [ "$program" = "$recorded_program" ] ||
    refuse program-mismatch "the export's program is ${program:-absent} and $identity_file records ${recorded_program:-none}"
source_sha=$(sha "$export_path/src/lib.rs")
recorded_source=$(sed -n '/"path": "src\/lib.rs",/{n;s/^ *"sha256": "\([0-9a-f]\{64\}\)",$/\1/p;}' "$root/$identity_file")
[ -n "$recorded_source" ] && [ "$source_sha" = "$recorded_source" ] ||
    refuse source-mismatch "src/lib.rs is $source_sha and $identity_file records ${recorded_source:-none} for the core of the conformance build"

# 3. The version asked for.
case $version_flag in
    release) version=$release ;;
    auto)
        if ! git -C "$root" rev-parse --git-dir >/dev/null 2>&1; then
            refuse shallow-clone "$root is not a git checkout: give the count with --dev N"
        fi
        [ "$(git -C "$root" rev-parse --is-shallow-repository)" = false ] ||
            refuse shallow-clone 'the clone is shallow, so the commit count is not the same in every clone: fetch the history or give the count with --dev N'
        version=$release-dev.$(git -C "$root" rev-list --first-parent --count HEAD)
        ;;
    *) version=$release-dev.$version_flag ;;
esac

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT HUP INT TERM
# cargo must not find a repository around the stage (a TMPDIR inside a checkout).
GIT_CEILING_DIRECTORIES=$stage
export GIT_CEILING_DIRECTORIES

# The files of the crate: the export's program files, the licences and README, and the
# export's manifest with `publish = false` replaced by the publication metadata (LexLean
# writes publish = false into every package) and, for a pre-release, the version.
stage_package() {
    directory=$1
    mkdir -p "$directory/src"
    cp "$export_path/package.json" "$export_path/provenance.json" "$directory/"
    cp "$export_path/src/lib.rs" "$directory/src/lib.rs"
    cp "$root/$publication_dir/README.md" "$directory/README.md"
    cp "$root/LICENSE-MIT" "$root/LICENSE-APACHE" "$directory/"
    sed -e "s/^version = \"$release\"\$/version = \"$version\"/" \
        -e "/^publish = false\$/{
a\\
description = \"$description\"\\
license = \"$license\"\\
repository = \"$repository\"\\
homepage = \"$homepage\"\\
readme = \"README.md\"\\
include = [\"src/**\", \"Cargo.toml\", \"package.json\", \"provenance.json\", \"README.md\", \"LICENSE-MIT\", \"LICENSE-APACHE\"]
d
}" "$export_path/Cargo.toml" >"$directory/Cargo.toml"
}

# Package the staged files into TARGET/package/<crate>-<version>.crate, offline. The build
# of the crate is `--dry-run`'s: the lints of the manifest are the pinned toolchain's.
package() {
    (cd "$1" && CARGO_TARGET_DIR=$2 cargo package --offline --no-verify --allow-dirty >/dev/null 2>"$2.log") || {
        cat "$2.log" >&2
        refuse not-reproducible "cargo package failed"
    }
}

archive_name=$crate-$version.crate
stage_package "$stage/files"
package "$stage/files" "$stage/target-a"
package "$stage/files" "$stage/target-b"
cmp -s "$stage/target-a/package/$archive_name" "$stage/target-b/package/$archive_name" ||
    refuse not-reproducible 'two packagings of the same files are different bytes'
derived=$stage/target-a/package/$archive_name

# The files of an archive, extracted: DIR/<crate>-<version>/...
extract() {
    mkdir -p "$2"
    tar -xzf "$1" -C "$2"
}

# The expected files, from the derived package, and the checks of the export against it.
extract "$derived" "$stage/expected"
expected=$stage/expected/$crate-$version
for file in package.json provenance.json src/lib.rs; do
    cmp -s "$expected/$file" "$export_path/$file" ||
        refuse file-tampered "$file of the package is not the export's"
done
for file in LICENSE-MIT LICENSE-APACHE; do
    cmp -s "$expected/$file" "$root/$file" ||
        refuse file-tampered "$file of the package is not the repository's"
done
(cd "$expected" && find . -type f | sort) >"$stage/expected.list"

# An archive is the derived package, file by file.
check_archive() {
    archive=$1
    [ -f "$archive" ] || refuse input-missing "$archive is not a file"
    tops=$(tar -tzf "$archive" | sed 's#/.*##' | sort -u)
    if [ "$tops" != "$crate-$version" ]; then
        case $tops in
            "$crate"-*) refuse version-mismatch "the archive is $tops and the version asked for is $version" ;;
            *) refuse name-mismatch "the archive holds $tops and the crate is $crate" ;;
        esac
    fi
    rm -rf "$stage/found"
    extract "$archive" "$stage/found"
    found=$stage/found/$crate-$version
    (cd "$found" && find . -type f | sort) >"$stage/found.list"
    diff "$stage/expected.list" "$stage/found.list" >"$stage/list.diff" ||
        refuse file-set "the archive does not hold exactly the files of the package: $(tr '\n' ' ' <"$stage/list.diff")"
    for file in Cargo.toml Cargo.toml.orig Cargo.lock; do
        cmp -s "$expected/$file" "$found/$file" ||
            refuse manifest-differs "$file of the archive is not the export's manifest with the publication metadata: $(diff "$expected/$file" "$found/$file" | head -n 6 | tr '\n' ' ')"
    done
    for file in package.json provenance.json src/lib.rs README.md LICENSE-MIT LICENSE-APACHE; do
        cmp -s "$expected/$file" "$found/$file" ||
            refuse file-tampered "$file of the archive is not the one the tree gives"
    done
}

report() {
    printf 'crate %s %s\n' "$crate" "$version"
    printf 'cargo %s (rust-toolchain.toml pins %s)\n' "$(cargo --version)" "${channel:-nothing}"
    printf 'program %s = %s program_sha256\n' "$program" "$identity_file"
    printf 'src/lib.rs %s = %s package.files src/lib.rs\n' "$source_sha" "$identity_file"
    printf 'sha256 %s  %s\n' "$(sha "$1")" "$archive_name"
}

if [ -n "$verify" ]; then
    check_archive "$verify"
    report "$verify"
    printf 'verified %s\n' "$verify"
    exit 0
fi

check_archive "$derived"

if [ "$dry_run" = true ]; then
    # The files cargo verifies and would send are the derived package's, file by file (cargo
    # does not keep the archive of a dry run). This compiles the crate and asks the registry
    # whether the name and version can be published; it uploads nothing, needs no credential.
    (cd "$stage/files" && CARGO_TARGET_DIR=$stage/target-dry cargo publish --dry-run --allow-dirty) ||
        refuse dry-run-differs 'cargo publish --dry-run failed'
    diff -r -x .cargo-ok "$expected" "$stage/target-dry/package/$crate-$version" >"$stage/dry.diff" ||
        refuse dry-run-differs "the dry run verified other files than the derived package: $(head -n 6 "$stage/dry.diff" | tr '\n' ' ')"
fi

if [ -n "$keep" ]; then
    [ ! -e "$keep" ] || refuse input-missing "$keep exists"
    cp -R "$stage/files" "$keep"
fi
mkdir -p "$out"
cp "$derived" "$out/$archive_name"
printf '%s  %s\n' "$(sha "$derived")" "$archive_name" >"$out/$archive_name.sha256"
report "$derived"
printf 'archive %s/%s\n' "$out" "$archive_name"
