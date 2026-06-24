# Slang benchmarks

This folder contains different benchmarks that allow us to keep track of how Slang compares to other libraries, or how it compares to itself over time.

We have two crates, one for each supported platform:

## [`cargo`](./cargo/)

The cargo crate contains three different entry points, each serving its purpose:

- [`benches/comparison`](./cargo/benches/comparison/main.rs): Runs the parsing process in different libraries for a number of [projects], and track the number of cycles/instructions and memory consumption with callgrind.
- [`benches/slang`](./cargo/benches/slang/main.rs): Runs different processes, like parsing, binding, cursor, etc. for just a couple of [projects]. It also uses callgrind.
- [`src/lib.rs`](./cargo/src/lib.rs): This library contains the code that is tested in the benches, and it adds smoke tests to ensure that the libraries being tested (including Slang) are working as expected.

[projects]: #projects

### Callgrind vs DHAT

Every cargo benchmark always runs [`callgrind`](https://kcachegrind.github.io/html/Home.html) (instructions, cache hits, estimated cycles). It can additionally run [`DHAT`](https://valgrind.org/docs/manual/dh-manual.html) for heap-allocation metrics, but DHAT is much slower, so we don't always run it:

- On `main` (and plain local / `--dry-run` runs): all three suites run DHAT.
- On PR benchmarks (`--pr-benchmark`, triggered by the `ci:perf` label): only the `slang_v2` suite runs DHAT (it's fast enough). The slower `slang` (v1) and `comparison` suites run callgrind only.
- On full PR benchmarks (`--pr-benchmark=full`, triggered by the `ci:perf:full` label): the `slang` and `comparison` suites also run DHAT, matching `main`. (`slang_v2` is fast enough that its workflow always passes `--pr-benchmark=full`, so it runs DHAT on every PR regardless of the `ci:perf:full` label.)

DHAT accuracy depends on its `num-callers` argument (see [`config.rs`](./cargo/src/config.rs)): `comparison` and `slang_v2` use the maximum (`500`) for accurate attribution, while `slang` (v1) is slow enough that it intentionally uses a smaller value (`12`) and accepts less accurate measurements.

Whether DHAT runs on a given PR is therefore a per-suite CI decision: each workflow picks `--pr-benchmark` (standard, Callgrind only) or `--pr-benchmark=full`. `infra perf cargo` just maps the mode to DHAT on/off by setting the `SLANG_PERF_SKIP_DHAT` environment variable on the benchmark process (a CLI flag on the bench binary isn't possible, since `iai-callgrind`'s `main!` owns argument parsing).

## [`npm`](./npm/)

In the `cargo` benchmarks, we use metrics that are mostly resistant from interference from outside (like other libraries running on the same process). When testing with `npm`, we can't have that, so instead, for each project we run each library in its own process, orchestrated by a main process in Rust ([`src/main.rs`](./npm/src/main.rs)). This process collects the results and sends it back to infra to ship it to bencher.

The main process calls the node application in [`src/benchmarks`](./npm/src/benchmarks/main.mts). This app has different _subjects under tests_ (each library). For Slang, we return the total time, but we also break it in parsing, binding db creation, binding resolution, etc. For the other libraries we use the total time.

Since we're measuring clock time, we cold-run a couple of times, and take the average of the following runs.

## Projects

The configuration file [`projects.json`](./projects.json) contains the Solidity projects and files that are tested in the above benchmarks. The source for these projects correspond to contracts in Ethereum Mainnet, and they are downloaded from Sourcify (see [`fetch.rs`](./utils/src/fetch.rs)). It's split in two sections:

- `projects`: This key contains full Solidity projects. The intention is to test the main entry point as defined by the project, and follow from there all of the dependencies.
- `files`: This key contains specific files to be tested, that are intended to be self-contained. Each file might or might not be the entrypoint of the Solidity project referred by the hash; for that reason, the entrypoint it must be specified with the `entrypoint` key.
