# Slang benchmarks

This folder contains different benchmarks that allow us to keep track of how Slang compares to other libraries, or how it compares to itself over time.

We have two crates, one for each supported platform:

## [`cargo`](./cargo/)

The cargo crate contains three benchmark entry points (plus a shared library), each serving its purpose:

- [`benches/comparison`](./cargo/benches/comparison/main.rs): Runs the parsing process in different libraries for a number of [projects], and track the number of cycles/instructions and memory consumption with callgrind.
- [`benches/slang`](./cargo/benches/slang/main.rs): Runs different processes, like parsing, binding, cursor, etc. for just a couple of [projects]. It also uses callgrind.
- [`benches/slang_v2`](./cargo/benches/slang_v2/main.rs): Runs different processes, like parsing, ir_builder, semantic, etc. for just a couple of [projects]. It also uses callgrind.
- [`src/lib.rs`](./cargo/src/lib.rs): This library contains the code that is tested in the benches, and it adds smoke tests to ensure that the libraries being tested (including Slang) are working as expected.

The cargo benchmarks use `valgrind`'s tools [Callgrind and DHAT](#callgrind-and-dhat) to measure
individual instructions and track memory usage. This is a very reliable way to measure performance,
since it's not affected by other processes or external factors. However, it has some details that
are worth mentioning, they're described in the [Callgrind and DHAT](#callgrind-and-dhat) section.

[projects]: #projects

### Benchmark naming

Each cargo benchmark identifier has the shape:

`{file}::{group}::{function} <project>:(<arguments>)`

- `{file}` is the bench binary: `slang`, `slang_v2`, or `comparison`.
- `{group}` is one of `cst`, `bindings`, or `v2_binder` for `slang`; `pipeline` for `slang_v2`; or `parsers` for `comparison`.
- `{function}` is the pipeline stage (`parser`, `cursor`, `query`, `bindings_build`, …) for `slang` / `slang_v2`, or the parser implementation (`slang`, `slang_v2`, `solar`, `tree_sitter`) for `comparison`.
- `<project>` is one of the entries in [`projects.json`](./projects.json), attached as a `#[bench::<project>("<project>")]` attribute on the function. Functions only list the projects they actually support; in `comparison`, that's how per-parser exclusions (e.g., tree-sitter cannot parse `uniswap`) are expressed.

For example, the name of the Slang V2, IR builder benchmark over the `merkle_proof` project would be `slang_v2::pipeline::ir_builder merkle_proof:(tests :: slang_v2 :: ir_builder :: setup("merkle_...`. The Slang V1 binder-resolve benchmark for the same project would be `slang::bindings::resolve merkle_proof:(...)`.

**Note:** Since `bencher` uses the text output of `gungraun`, names are truncated (as shown above).

### Callgrind vs DHAT

Every cargo benchmark always runs [`callgrind`](https://kcachegrind.github.io/html/Home.html) (instructions, cache hits, estimated cycles). It can additionally run [`DHAT`](https://valgrind.org/docs/manual/dh-manual.html) for heap-allocation metrics, but DHAT is much slower, so we don't always run it:

- On `main` (and plain local / `--dry-run` runs): all three suites run DHAT.
- On PR benchmarks (`--pr-benchmark`, triggered by the `ci:perf` label): only the `slang_v2` suite runs DHAT (it's fast enough). The slower `slang` (v1) and `comparison` suites run callgrind only.
- On full PR benchmarks (`--pr-benchmark=full`, triggered by the `ci:perf:full` label): the `slang` and `comparison` suites also run DHAT, matching `main`. (`slang_v2` is fast enough that its workflow always passes `--pr-benchmark=full`, so it runs DHAT on every PR regardless of the `ci:perf:full` label.)

DHAT accuracy depends on its `num-callers` argument (see [`config.rs`](./cargo/src/config.rs)): `comparison` and `slang_v2` use the maximum (`500`) for accurate attribution, while `slang` (v1) is slow enough that it intentionally uses a smaller value (`12`) and accepts less accurate measurements.

Whether DHAT runs on a given PR is therefore a per-suite CI decision: each workflow picks `--pr-benchmark` (fast, Callgrind only) or `--pr-benchmark=full`. `infra perf cargo` just maps the mode to DHAT on/off by setting the `SLANG_PERF_SKIP_DHAT` environment variable on the benchmark process (a CLI flag on the bench binary isn't possible, since `iai-callgrind`'s `main!` owns argument parsing).

## [`npm`](./npm/)

In the `cargo` benchmarks, we use metrics that are mostly resistant from interference from outside (like other libraries running on the same process). When testing with `npm`, we can't have that, so instead, for each project we run each library in its own process, orchestrated by a main process in Rust ([`src/main.rs`](./npm/src/main.rs)). This process collects the results and sends it back to infra to ship it to bencher.

The main process calls the node application in [`src/benchmarks`](./npm/src/benchmarks/main.mts). This app has different _subjects under tests_ (each library). For Slang, we return the total time, but we also break it in parsing, binding db creation, binding resolution, etc. For the other libraries we use the total time.

Since we're measuring clock time, we cold-run a couple of times, and take the average of the following runs.

## Projects

The configuration file [`projects.json`](./projects.json) contains the Solidity projects and files that are tested in the above benchmarks. The source for these projects correspond to contracts in Ethereum Mainnet, and they are downloaded from Sourcify (see [`fetch.rs`](./utils/src/fetch.rs)). It's split in two sections:

- `projects`: This key contains full Solidity projects. The intention is to test the main entry point as defined by the project, and follow from there all of the dependencies.
- `files`: This key contains specific files to be tested, that are intended to be self-contained. Each file might or might not be the entrypoint of the Solidity project referred by the hash; for that reason, the entrypoint it must be specified with the `entrypoint` key.

## Callgrind and DHAT

Callgrind and DHAT are tools from the [Valgrind](https://valgrind.org/docs/manual/manual-intro.html)
framework.
Callgrind is a call-graph generating cache profiler, it tracks functions and their provenance, it can
be used to measure number of instructions, but also where those instructions come from.
DHAT is a heap profiler that tracks memory allocations and deallocations, by tagging them with the stack trace of the allocation.

We run these tools through the [`gungraun`](https://github.com/gungraun/gungraun) crate, which
is a benchmarking framework for Rust.

Because of the peculiarities of these tools, we have to take some care on how these benchmarks are run.

### How Callgrind works

Callgrind works by instrumenting the binary of the application, tracking every call and return and building a call graph of the
trace of the application. It reports how many instructions were executed, their relationship to the source code, and cache simulation.

The instruction count is exact and machine-independent, which is what makes these benchmarks reproducible across machines.
The cache-hit and estimated-cycle figures, on the other hand, come from a simulated machine model (a generic cache hierarchy)
rather than the real CPU the benchmark runs on, so they're best read as relative indicators rather than absolute truth.

More information can be found in the [Callgrind manual](https://valgrind.org/docs/manual/cl-manual.html).

### How DHAT works

DHAT is a heap profiler that tracks memory allocations and deallocations, together with memory accesses.
DHAT tags every memory block with the stack trace at the moment of the allocation, that is, two blocks of memory allocated
at the same stack trace will be merged together.

Furthermore, DHAT tags memory accesses not by where they happen, but by where the memory that's being accessed was allocated,
which is very helpful to answer questions like "how is my program using the chunk of memory allocated at this point in the code?"

More information can be found in the [DHAT manual](https://valgrind.org/docs/manual/dh-manual.html).

### How gungraun works

`gungraun` launches a new process for each benchmark, and runs the benchmark through `valgrind` (including any `setup` or `teardown` stages).
It collects the output of the different tools and processes it, so that what we report reflects only the benchmark function and not the
non-benchmark stages (like `setup`). Callgrind and DHAT achieve this in fundamentally different ways.

With Callgrind this is straightforward, and it needs no extra filtering at all. `gungraun` runs Callgrind with `--toggle-collect`
pointed at the benchmark function, so Callgrind starts with event collection turned off and
only counts instructions while execution is inside the benchmark function. The whole process is still _executed_ under Callgrind, but the
instructions in `setup`/`teardown` are never _counted_, and the reported total is simply Callgrind's `summary`/`totals` of collected events.

With DHAT, this is a bit more complicated, since the information is only tagged with the stack trace of the allocation, and we have to filter out the allocations that happen outside of the benchmark function.
The stack trace is not gathered by DHAT directly, but by `valgrind` itself, and the depth it records is bounded by the `--num-callers` option (which defaults to 12 and can be set to at most 500). See [the pitfall below](#valgrinds-num-callers-and-dhat-filtering) for why this matters.

### Pitfalls and common misconceptions

#### Reads of external memory

Because DHAT tracks memory accesses by the point of allocation, and `gungraun` filters out allocations using these tags, if a bit of memory
is allocated within the `setup` stage (e.g. the `CST` when benchmarking `ir_builder`), we don't track reads/writes to that memory within the
benchmark.

#### External reads of memory

Similar to above, if we read memory allocated in the actual benchmarking function outside of it (for example in a `teardown` stage),
we count those accesses.

What this means is, DHAT tracks how the memory allocated within the benchmarking function is used, regardless of where that use happens.

#### t-gmax may not be what you think

t-gmax is a single whole-process instant: the moment the total heap size reaches its global maximum. `gungraun` reports how many of the
bytes allocated inside the benchmarking function were still alive _at that instant_ — not a per-function peak.
This value should be meaningless to us: in some cases the `setup` stage allocates a lot more memory than the benchmark itself,
so the global peak happens before our benchmarking function starts (when none of our blocks exist yet), and we get a value of 0.

#### valgrind's `num-callers` and DHAT filtering

Because DHAT uses `valgrind`'s own system for tracking stack traces, the depth it records is bounded by the `--num-callers` option.
`valgrind` defaults this to 12 and caps it at 500; the deep-pipeline benches (`slang_v2` and `comparison`) raise it to the maximum,
500 (see [`config.rs`](./cargo/src/config.rs)), because slang's bench-body call chains (parser, IR builder, semantic) can run much
deeper than the default 12 frames.

`gungraun` decides whether an allocation belongs to the benchmark by looking for the bench function (the wrapper frame it injects)
in the allocation's recorded stack trace. If that frame sits deeper than `num-callers`, it falls off the recorded trace and `gungraun`
drops the allocation from the filtered results, even though it belongs in scope. Raising `num-callers` therefore tends to _increase_
the reported allocation totals (more deep frames get attributed in scope). Since 500 is `valgrind`'s hard maximum, there is no headroom
left if a chain ever runs deeper, so this filter remains inherently fragile.

Conversely, `slang` (v1) deliberately stays at the default of 12: raising it makes its already-slow benchmarks time out in CI.
Because v1's bench-body chains can run deeper than 12 frames, this means its DHAT allocation totals are knowingly under-attributed
(some in-scope allocations fall off the trace and are dropped); we accept that inaccuracy on v1 rather than the slowdown.

#### Non-determinism

Another gotcha we have to be careful about is non-determinism. Traditional benchmarks are run
multiple times in an attempt to recover average metrics, minimizing the effect of external
and internal factors.
One of many sources of factors (especially internal) is non-deterministic abstraction,
for example, `HashMap`s that use a random seed for their internal hashing function.

With `gungraun` we don't need (and we shouldn't) run the benchmarks multiple times,
since the measurements should have no external interference, and the benchmarks themselves
are quite slow.
However, internal non-determinism can still affect measurements, and we have to be careful about it.

To try and avoid these kinds of situations, [we're keeping](https://github.com/NomicFoundation/slang/pull/1850)
internal collections deterministic.
The benchmarks themselves should also be deterministic, for example, source files should
be processed in a [deterministic order](https://github.com/NomicFoundation/slang/pull/1583).

#### State of the system before the benchmark

Although determinism within the benchmark itself is important, we also need to be careful about
non-determinism outside (like in a `setup` stage).

For example, in [this case](https://github.com/NomicFoundation/slang/pull/1858) a `HashMap`
used when loading manifest dependencies, and dropped before
the actual benchmark, would leave the free-list in different states, essentially affecting
how the memory allocator behaves when the benchmark runs, and affecting the measurements.
