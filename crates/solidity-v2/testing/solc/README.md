# `solidity_v2_testing_solc`

Runs **slang (v2)** against `solc`'s own [`libsolidity` semantic test
suite](https://github.com/argotorg/solidity/tree/develop/test/libsolidity/semanticTests),
checking that all of this **valid** Solidity still compiles without slang
emitting any error diagnostics.

It does this for **every Solidity version slang v2 supports** (0.8.0 up to the
latest): for each version it reads the semantic tests from that version's `solc`
release tag and runs slang against them pinned to that same language version.

## Usage

The whole `(version, test)` matrix is a **single test**, and runs as part of the
regular `infra test` (and, in turn, `infra ci`) like any other:

```sh
infra test cargo solc_semantic_suite

# Check locally without rewriting the baseline (as CI does):
CI=1 infra test cargo solc_semantic_suite
```

The first run clones `solc`'s repository into `target/solc-comparison/`; every
run after that is offline and takes a couple of seconds. Nothing else is left
behind — the per-version checkouts live in a temporary directory for the
duration of the run.

The run fails if **any** test fails without being an
[expected failure](#expected-failures), and separately if the checked-in results
are **stale** (a listed pair now passes, or an expected failure does). The
results file is written before either check, so a red run still leaves an
up-to-date record of what's left. The diagnostics behind the first few
newly-failing tests are printed, so a regression points straight at its cause.

## How it works

The ~50k `(version, test)` pairs run in a couple of seconds in-process across
`rayon`, so they're deliberately **one** `#[test]` rather than one test each —
splitting them up would only cost `nextest` tens of thousands of processes.

1. **Fetch** — a single bare clone of `argotorg/solidity`, kept at
   `target/solc-comparison/solidity.git`, serves every version. Each version's
   `semanticTests/` tree is checked out of it at that version's release tag (e.g.
   `v0.8.20`). The first run clones; every run after that is offline.

    One clone rather than a snapshot per tag: the tags share nearly all of their
    content, so a delta-compressed pack is smaller than 37 separate archives
    would be, and — unlike a partial (`--filter=blob:none`) clone — it holds
    every blob, so reading any tag afterwards needs no network at all. Release
    tags are immutable, so the only thing that triggers a fetch is a tag the
    local clone doesn't have yet.

    **The checkouts go to a temporary directory**, removed when the run ends. Only
    the clone is worth keeping between runs (it's the part that would otherwise be
    re-downloaded, and it's what CI caches); re-checking out all 37 versions from
    it takes well under a second, which isn't worth a few hundred megabytes of
    cached scratch space.

    **Tags are mutable in principle, so we pin the commit.** Each version records
    the commit its tag resolved to in the results file below, so if one is ever
    re-pointed the change lands in the diff right next to the counts it
    invalidates.

2. **Parse** — each test file is in the `isoltest` format.
   We parse out the sources and the `EVMVersion` setting; the runtime
   expectations are ignored. An `ExternalSource: <source name>=<path>` directive
   loads the fixture at `<path>` but names the source unit after the left-hand
   side, which is a source _name_ and not an import remapping.
3. **Run** — each `(version, test)` pair compiles with the slang v2
   `CompilationUnit::create` pinned to that language version and the resolved EVM
   target (the `EVMVersion` setting if present, else that version's default),
   resolving imports with the shared `solidity_v2_testing_utils` `path_resolver`,
   which follows the rules `solc` applies to a standard JSON input with no
   remappings.

    A setting we can't honor — an EVM target name we don't know, or a constraint
    no supported target satisfies — fails the run rather than falling back to the
    default. Silently analyzing at the wrong target would bake a misleading
    result into the baseline.

    `isoltest`'s `@future` placeholder is the one exception: it names an EVM
    version that hasn't been released, so there is no target to analyze at and
    the _test_ fails rather than the run. That's a fact about the test, not a gap
    in this code, so it belongs in the expected failures below rather than in an
    error.

4. **Baseline** — everything the run produced is reconciled with
   [`results.generated.json`](./results.generated.json): checked in CI, rewritten
   locally. Each version records the commit its tests came from, how many ran,
   how many passed and failed, and which ones failed:

    ```json
    "0.8.0": {
        "commit": "c7dfd78e57c5ad7abd485dc1cc13d8f0ab09d431",
        "executed": 1045,
        "passed": 1043,
        "expected_failures": 2,
        "unexpected_failures": 0,
        "failures": []
    }
    ```

    Both of this version's failures are expected, which is why `failures` is
    empty and `expected_failures` accounts for them — see below.

    `executed` and `passed` are derivable from `failures`, but recording them
    makes the diff catch the dataset itself changing size — a version whose test
    count moves is worth noticing, and would otherwise be invisible whenever the
    new tests happen to pass. `expected_failures` counts the ones declared in
    [`src/expected_failures.rs`](./src/expected_failures.rs), which are left out
    of `failures`, so the tally stays
    `executed == passed + expected_failures + unexpected_failures`.

## Expected failures

A failure we actually stand behind (slang deliberately stricter or looser than
`solc`) goes in [`src/expected_failures.rs`](./src/expected_failures.rs)
instead, next to its reason.

Entries are grouped by that reason rather than by test, since one deliberate
difference usually covers a set of them. Each case names **one** test file —
never a directory — and the `LanguageVersionSpecifier` range it fails over:

```rust
ExpectedFailures {
    reason: "Explain what slang does differently, and why that is correct.",
    cases: &[
        ExpectedCase {
            test_path: "revertStrings/empty_v1.sol",
            versions: LanguageVersionSpecifier::from(LanguageVersion::V0_8_0),
        },
        ExpectedCase {
            test_path: "revertStrings/something_that_fails_until_0_8_5.sol",
            versions: LanguageVersionSpecifier::till(LanguageVersion::V0_8_5),
        },
    ],
}
```
