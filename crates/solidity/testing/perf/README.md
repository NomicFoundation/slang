# Slang benchmarks

This folder contains different benchmarks that allow us to keep track of how Slang compares to other libraries, or how it compares to itself over time.

We have two crates, one for each supported platform:

## [`cargo`](./cargo/)

The cargo crate contains three different entry points, each serving its purpose:

- [`benches/comparison`](./cargo/benches/comparison/main.rs): Runs the parsing process in different libraries for a number of [projects], and track the number of cycles/instructions and memory consumption with callgrind.
- [`benches/slang`](./cargo/benches/slang/main.rs): Runs different processes, like parsing, binding, cursor, etc. for just a couple of [projects]. It also uses callgrind.
- [`src/lib.rs`](./cargo/src/lib.rs): This library contains the code that is tested in the benches, and it adds smoke tests to ensure that the libraries being tested (including Slang) are working as expected.

[projects]: #projects

## [`npm`](./npm/)

In the `cargo` benchmarks, we use metrics that are mostly resistant from interference from outside (like other libraries running on the same process). When testing with `npm`, we can't have that, so instead, for each project we run each library in its own process, orchestrated by a main process in Rust ([`src/main.rs`](./npm/src/main.rs)). This process collects the results and sends it back to infra to ship it to bencher.

The main process calls the node application in [`src/benchmarks`](./npm/src/benchmarks/main.mts). This app has different _subjects under tests_ (each library). For Slang, we return the total time, but we also break it in parsing, binding db creation, binding resolution, etc. For the other libraries we use the total time.

Since we're measuring clock time, we cold-run a couple of times, and take the average of the following runs.

## Projects

The configuration file [`projects.json`](./projects.json) contains the Solidity projects and files that are tested in the above benchmarks. It's split in two sections:

- `projects`: This key contains full Solidity projects. The intention is to test the main entry point as defined by the project, and follow from there all of the dependencies.
- `files`: This key contains specific files to be tested, that are intended to be self-contained. This files might or might not be the entry point of the Solidity project referred by the hash, and for that reason they must override it with the `entrypoint` key.
