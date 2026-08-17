# `solidity_testing_solc_comparison`

Runs **slang (v2)** against `solc`'s own [`libsolidity` semantic test
suite](https://github.com/argotorg/solidity/tree/develop/test/libsolidity/semanticTests),
checking that all of this **valid** Solidity still compiles without slang
emitting any error diagnostics.

It does this for **every Solidity version slang v2 supports** (0.8.0 up to the
latest): for each version it reads the semantic tests from that version's `solc`
release tag and runs slang against them pinned to that same language version.

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
   `CompilationBuilder` pinned to that language version and the resolved EVM
   target (the `EVMVersion` setting if present, else that version's default),
   resolving imports with the shared `solidity_testing_utils` `ImportResolver`.
4. **Baseline** — everything the run produced is reconciled with
   [`results.generated.json`](./results.generated.json): checked in CI, rewritten
   locally. Each version records the commit its tests came from, how many ran,
   how many passed and failed, and which ones failed:

    ```json
    "0.8.7": {
        "commit": "6da8b019757383bcc85be6a3f7ecc2fb4c65f5f2",
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
