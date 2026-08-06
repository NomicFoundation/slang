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

The whole `(version, test)` matrix is a **single test**, run as part of the
regular `infra test` (and, in turn, `infra ci`) as its own step.

Because it fetches an external dataset, the test is marked `#[ignore]`: no plain
`cargo test` or `cargo nextest run` will reach for the network, whether or not
it goes through `infra`. Running it is always an explicit opt-in.

```sh
# Run the whole suite (all versions). In CI this checks against the committed
# baseline; run locally it regenerates it (see "Baseline update mode" below).
infra test solc-semantic

# Check locally without rewriting the baseline (as CI does):
CI=1 infra test solc-semantic
```

The run fails if any test **regresses** (fails without being in the baseline) or
if the baseline is **stale** (a listed pair now passes). This is what makes it a
CI regression guard. The diagnostics behind the first few newly-failing tests are
printed, so a regression points straight at its cause.

## How it works

The ~60k `(version, test)` pairs run in a couple of seconds in-process across
`rayon`, so they're deliberately **one** `#[test]` rather than one test each —
splitting them up would only cost `nextest` tens of thousands of processes.

1. **Fetch** — for every supported version (`LanguageVersion::ALL`), download the
   `argotorg/solidity` tarball at that version's release tag (e.g. `v0.8.20`) and
   extract the `semanticTests/` tree into `target/solc-comparison/<tag>/`.

    **Tags are mutable, so we pin the commit.** Each tarball's `pax_global_header`
    carries the commit SHA the tag resolved to; we record it (in
    [`pinned-commits.generated.json`](./pinned-commits.generated.json), a
    `{ "<version>": "<sha>" }` map) when the baseline is generated, and check it
    on every run. If a tag is later re-pointed at a different commit, the run
    fails loudly rather than silently testing against changed content. (A git commit SHA is itself a
    content hash, so this subsumes a separate checksum — no extra download needed.)

2. **Parse** — each test file is in the `isoltest` format.
   We parse out the sources and the `EVMVersion` setting; the runtime
   expectations are ignored.
3. **Run** — each `(version, test)` pair compiles with the slang v2
   `CompilationBuilder` pinned to that language version and the resolved EVM
   target (the `EVMVersion` setting if present, else that version's default),
   resolving imports with the shared `solidity_testing_utils` `ImportResolver`.
4. **Baseline** — the set of pairs slang rejected is reconciled with
   `results.generated.json`: checked in CI, rewritten locally (see "Baseline
   update mode"). Each version records how many tests ran, how many passed and
   failed, and which ones failed:

    ```json
    "0.8.7": {
        "executed": 1205,
        "passed": 1203,
        "failed": 2,
        "failures": ["experimental/stub.sol", "experimental/type_class.sol"]
    }
    ```

    `executed` and `passed` are derivable from `failures`, but recording them
    makes the diff catch the dataset itself changing size — a version whose test
    count moves is worth noticing, and would otherwise be invisible whenever the
    new tests happen to pass.
