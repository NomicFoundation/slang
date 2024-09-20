//! WARNING:
//! The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
//! For the function below: `iai::benchmarks::list_contracts`
//! Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.

#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unit_arg)]

mod dataset;

use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Direction, FlamegraphConfig,
    LibraryBenchmarkConfig, Tool, ValgrindTool,
};
use slang_solidity::bindings::Bindings;
use slang_solidity::cst::Node;

#[library_benchmark]
fn parser() {
    black_box(dataset::run_parser());
}

#[library_benchmark(setup = dataset::run_parser)]
fn cursor(trees: Vec<Node>) {
    black_box(dataset::run_cursor(&trees));
}

#[library_benchmark(setup = dataset::run_parser)]
fn query(trees: Vec<Node>) {
    black_box(dataset::run_query(&trees));
}

#[library_benchmark(setup = dataset::run_parser)]
fn build_bindings(trees: Vec<Node>) {
    black_box(dataset::run_build_bindings(&trees));
}

#[library_benchmark(setup = dataset::run_build_bindings)]
#[bench::run(args = (&dataset::run_parser()))]
fn resolve_references(bindings: Bindings) {
    black_box(dataset::run_resolve_references(&bindings));
}

library_benchmark_group!(
    name = benchmarks;

    benchmarks = parser, cursor, query, build_bindings, resolve_references
);

main!(
    config = LibraryBenchmarkConfig::default()
        // 'valgrind' supports many tools. By default, it runs 'callgrind', which reports these metrics:
        // https://kcachegrind.github.io/html/Home.html
        //
        // Instructions:            Total CPU instructions executed.
        // L1 Hits:                 Total (simulated) number of times the L1 cache was hit.
        // L2 Hits:                 Total (simulated) number of times the L2 cache was hit.
        // RAM Hits:                Total (simulated) number of times the RAM was hit.
        // Total read+write:        Total memory reads/writes during the entire execution.
        // Estimated Cycles:        Number of CPU cycles (estimated) that went by during the entire execution.
        //
        // We also enable the 'DHAT' tool below, which reports these metrics:
        // https://valgrind.org/docs/manual/dh-manual.html
        //
        // Total bytes:             How many bytes were allocated over the entire execution.
        // Total blocks:            How many heap blocks were allocated over the entire execution.
        // At t-gmax bytes:         How many bytes were alive when the heap size reached its global maximum (as measured in bytes).
        // At t-gmax blocks:        How many heap blocks were alive when the heap size reached its global maximum (as measured in bytes).
        // At t-end bytes:          How many bytes were alive at the end of execution (were not explicitly freed).
        // At t-end blocks:         How many heap blocks were alive at the end of execution (were not explicitly freed).
        // Reads bytes:             How many bytes within heap blocks were read during the entire execution.
        // Writes bytes:            How many bytes within heap blocks were written during the entire execution.
        .tool(Tool::new(ValgrindTool::DHAT))

        // This enables generating flame graphs into Cargo's 'target' directory.
        // They will be listed by 'infra perf' at the end of the run:
        .flamegraph(FlamegraphConfig::default().direction(Direction::BottomToTop))

        // 'valgrind' executes tests without any environment variables set by default.
        // Let's disable this behavior to be able to execute our infra utilities:
        .env_clear(false);

    library_benchmark_groups = benchmarks
);
