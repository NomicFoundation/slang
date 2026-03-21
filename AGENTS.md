# AGENTS.md

## Project Overview

Slang is a modular Solidity compiler tooling suite built by the Nomic Foundation. It provides a full-fidelity concrete syntax tree (CST) parser, semantic analysis, and binding computation for Solidity source code. It is **not** a replacement for solc — it focuses on code analysis and developer tooling, not bytecode generation.

- **Language**: Rust workspace (60 crates) with TypeScript/npm bindings via WASM
- **Supports**: Solidity versions 0.4.11 to latest 0.8.+
- **Published as**: `slang_solidity` (crates.io), `@nomicfoundation/slang` (npm)

Additionally, the repository also contains the crates for the under-development Slang v2, which for now is written in Rust (no TypeScript) and supports only versions starting from 0.8.0. The v2 implementation is a complete rewrite of the v1 codebase, with a new LALRPOP-based parser and redesigned architecture. Both v1 and v2 live in the same branch, in adjacent crates/packages.

## Repository Architecture

```tree
crates/
├── infra/              Build tooling and the infra CLI
├── codegen/            Code generation from language definitions (v1)
├── codegen-v2/         Next-gen code generation (LALRPOP-based)
├── language/           Language definition building blocks (v1)
├── language-v2/        v2 language definitions
├── metaslang/
│   ├── cst/            Generic CST library (build, navigate, query)
│   ├── bindings/       Semantic binding computation
│   ├── graph_builder/  Binding graph construction
│   └── stack_graphs/   Stack graphs for scope analysis
├── solidity/
│   ├── inputs/language/
│   │   └── src/definition.rs   Grammar definition (hand-written v1 parser)
│   ├── outputs/
│   │   ├── cargo/crate/    Main published Rust crate (slang_solidity)
│   │   ├── cargo/cli/      CLI tool (slang_solidity_cli)
│   │   ├── cargo/wasm/     WASM bindings
│   │   ├── cargo/tests/    Integration tests
│   │   ├── npm/package/    npm package (@nomicfoundation/slang)
│   │   └── spec/           Language specification generator
│   └── testing/            Snapshots, perf, solc compat, sourcify tests
├── solidity-v2/
│   ├── inputs/language/
│   │   └── src/definition.rs   Grammar definition (LALRPOP-based v2 parser)
│   ├── outputs/
│   │   └── cargo/
│   │       ├── common/     Shared types (slang_solidity_v2_common)
│   │       ├── cst/        CST node types (slang_solidity_v2_cst)
│   │       ├── parser/     LALRPOP-generated parser (slang_solidity_v2_parser)
│   │       └── tests/      V2 integration tests
│   └── testing/
│       ├── snapshots/      CST output golden files (cst_output/)
│       └── utils/          Test utilities and V1 comparison tooling
└── documentation/      MkDocs documentation site
```

v1 and v2 implementations coexist in the same `main` branch.

## Hermit/Toolchain Binaries

**Always use Hermit-managed binaries from `./bin/`** instead of system `PATH` versions. This ensures correct, reproducible tool versions:

```sh
./bin/cargo check --workspace # NOT: cargo check --workspace
./bin/npm install             # NOT: npm install
```

Available binaries in `./bin/`: `cargo`, `node`, `npm`, `npx`, `pnpm`, `gh`, `python3`, `pip`, and more.

**Don't run linters/formatters directly**, but always use `./scripts/bin/infra lint` to ensure consistent configuration and versions.

## Build & Development Commands

The project uses a custom `infra` CLI that orchestrates all build operations. Use `--help` on any command or subcommand to discover options:

```sh
./scripts/bin/infra --help       # See all available commands
./scripts/bin/infra setup        # Install all dependencies
./scripts/bin/infra setup --help # See setup subcommands (cargo, npm, etc.)
./scripts/bin/infra check        # Codegen + cargo check + npm check + public API check
./scripts/bin/infra check --help # See check subcommands (codegen, cargo, npm, public-api)
./scripts/bin/infra test         # Run all tests (cargo nextest + jest)
./scripts/bin/infra test --help  # See test subcommands (cargo, npm) and passthrough args
./scripts/bin/infra lint         # Run all linters
./scripts/bin/infra lint --help  # See lint subcommands (markdownlint, rustfmt, yamllint, prettier, etc.)
./scripts/bin/infra ci           # Full CI run: setup + check + test + lint
```

Always run `infra setup` when initializing a new workspace, to ensure all tools/dependencies are available.

Individual lint subcommands can be run for quick checks (e.g., `./scripts/bin/infra lint yamllint` for YAML files, `./scripts/bin/infra lint rustfmt` for Rust formatting).

You can also use `nextest` directly for faster iteration on Rust tests:

```sh
./bin/cargo nextest run -p <crate_name>   # Test a single crate
./bin/cargo nextest run [FILTERS]         # Run tests that match specific filters (test names or file patterns)
```

## Code Generation

Many source files are auto-generated, and are either under a `/generated/` directory, or have `*.generated.*` in their name.

**Never edit these files directly.** Instead, modify the source definitions and regenerate using:

```sh
./scripts/bin/infra check codegen
```

Most of the generated files are produced from an accompanying `.jinja2` template next to them.
Despite the `.jinja2` file extension, they use [Tera](https://keats.github.io/tera/docs/) syntax — **not** Jinja2.

## Sync Markers

The codebase uses `__MARKER_NAME__` inline comment tags to flag values that must be kept in sync across multiple files.
Whenever you see this annotation, it means all occurrences of that marker must be updated together.

**When touching any marked value**, find all files that share the same marker and update them together:

```sh
grep -r "__MARKER_NAME__" .
```

## Snapshot Tests

For testing, we maintain snapshots checked into the repo:

- V1 Snapshots:
    - Parser: `crates/solidity/testing/snapshots/cst_output/`
    - Binder: `crates/solidity/testing/snapshots/bindings_output/`
- V2 Snapshots:
    - Parser: `crates/solidity-v2/testing/snapshots/cst_output/`

When source changes cause snapshot mismatches, the test output shows the diff.
Simply re-run the tests, and they will update the snapshot files on disk automatically.
Then commit the updated snapshots alongside your code change.

## Important Gotchas

- **Asking for help/more info**: If you need more context or have questions, use the `AskUserQuestion` tool to ask the user for clarification. Don't guess or make assumptions about project details.
- **Commit messages**: Unless instructed otherwise, use a short, concise title. No verbose description, and no information about the AI chat session or original prompt.
- **Keep `AGENTS.md` up to date**: If you learn new important information about the project, or if you notice any of the above information is outdated, please update this file with the new details. This is the single source of truth for all agents working on this project.
