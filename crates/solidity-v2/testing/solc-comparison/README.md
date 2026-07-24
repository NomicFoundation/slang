# `solidity_testing_solc_comparison`

Runs **slang (v2)** against `solc`'s own [`libsolidity` semantic test
suite](https://github.com/argotorg/solidity/tree/develop/test/libsolidity/semanticTests),
checking that all of this **valid** Solidity still compiles without slang
emitting any error diagnostics.

It does this for **every Solidity version slang v2 supports** (0.8.0 up to the
latest): for each version it downloads the semantic tests from that version's
`solc` release tag and runs slang against them pinned to that same language
version.

## Usage

This runs as part of `infra verify` (and, in turn, `infra ci`), as a step after
`infra test` in the CI pipeline. It is intentionally kept out of `infra test`,
since it fetches an external dataset.

```sh
# Run the whole suite (all versions). In CI this checks against the committed
# baseline; run locally it regenerates it (see "Baseline update mode" below).
infra verify

# cargo test args are forwarded — e.g. run just the 0.8.20 cases.
infra verify solc-semantic-suite -- v0.8.20/

# Check locally without rewriting the baseline (as CI does):
CI=1 infra verify
```

Each `(version, test)` pair is a separate test case, so a run fails if any test
**regresses** (fails without being in the baseline) or if the baseline is
**stale** (a listed pair now passes). This is what makes it a CI regression
guard.

### Baseline update mode

Like the repo's other snapshot tests, the mode is chosen by the `CI` env var:

- **In CI** (`CI` set) the cases **check** against the committed baseline and
  the run fails on any drift.
- **Run locally** (`CI` unset) the cases instead **rewrite** the baseline
  (`expected-failures.json`), and the fetch step re-pins `pinned-commits.json`.

So after intentionally changing which tests pass (a new validation, a parser
fix, a version bump), just run `infra verify solc-semantic-suite` locally and commit
the regenerated files.

## How it works

The suite is a [`datatest-stable`](https://github.com/nextest-rs/datatest-stable)
harness (`tests/semantic_tests.rs`, `harness = false`), run via `cargo test`.
`datatest-stable` generates **one test case per file**; we point its `root` at
a directory holding every version's tests
(`target/solc-comparison/v<version>/…`), so the generated cases span the whole
`(version, test)` matrix.

We run it with `cargo test` (in-process, threaded) rather than `cargo nextest`:
nextest is [process-per-test by design](https://nexte.st/docs/design/why-process-per-test/)
and not configurable otherwise, so spawning ~50k processes is both slow (minutes
vs seconds) and overwhelms nextest's list phase. `datatest-stable` supports both
runners, and the whole matrix runs in-process in seconds.

1. **Fetch** — the harness's `root` expression downloads, for every supported
   version (`LanguageVersion::ALL`), the `argotorg/solidity` tarball at that
   version's release tag (e.g. `v0.8.20`) and extracts the `semanticTests/`
   tree into `target/solc-comparison/<tag>/`, reusing the shared
   `infra_utils::http` download helper. The versions are fetched in parallel
   (via `rayon`), since a cold cache means three dozen independent network
   downloads. Release tags are immutable, so a populated cache is reused without
   hitting the network (and `target/` is cached in CI). Because the `root`
   expression fetches this whole dataset, the suite is excluded from the default
   `infra test` run (see `Cargo.toml`) and driven only by `infra verify`.

    **Tags are mutable, so we pin the commit.** Each tarball's `pax_global_header`
    carries the commit SHA the tag resolved to; we record it (in
    [`pinned-commits.json`](./pinned-commits.json), a `{ "<version>": "<sha>" }`
    map) when the baseline is generated, and verify it on every fetch. If a tag
    is later re-pointed at a different commit, the fetch fails loudly rather than
    silently testing against changed content. (A git commit SHA is itself a
    content hash, so this subsumes a separate checksum — no extra download needed.)

2. **Parse** — each test file is in the `isoltest` format: Solidity source
   (optionally split into multiple named sources via `==== Source: <name> ====`
   and referencing shared fixtures via `==== ExternalSource: <path> ====`),
   followed by a `// ====` settings block and a `// ----` runtime-expectation
   block. We parse out the sources and the `EVMVersion` setting; the runtime
   expectations are ignored.
3. **Run** — each case parses its `(version, test)` out of the file path,
   compiles with the slang v2 `CompilationBuilder` pinned to that language
   version and the resolved EVM target (the `EVMVersion` setting if present,
   else that version's default), resolving imports with the shared
   `solidity_testing_utils` `ImportResolver`. The case **passes** iff slang's
   result (clean / has-errors) matches the baseline for that `(version, test)`.
4. **Baseline** — in CI (checking) each case is compared to the baseline.
   Outside CI (update mode) the cases instead rewrite `expected-failures.json`,
   and the fetch step re-pins `pinned-commits.json` (see "Baseline update mode").
