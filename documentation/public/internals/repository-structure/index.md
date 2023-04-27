# Repository Structure

This repository is split into multiple projects at the root folder. Each project has its own dependencies and tools used to build, test, and ship different parts of the repository. For example, a Rust environment for the compiler, a Python environment for documentation, and a NodeJS environment for linters. This allows us to implement different binaries, APIs, and internal tools/scripts, and package/version them together, while having minimal inter-dependencies.

All dependencies should have exact full versions, and we can rely on tooling to automatically upgrade it over time. It allows us to have perfectly reproducible builds for every commit, [a critical aspect](https://github.com/dotnet/designs/blob/40794be63ecd8b35e9596412050a84dedd575b99/accepted/2020/reproducible-builds.md) for compilers, and developer tools in general.

## Directory Structure

Currently, the repository has the following projects:

-   `.changeset/`: pending user visible changes not released yet.
-   `.devcontainer/`: self-contained Docker image to develop, build, test, and publish.
-   `crates/`:
    -   `codegen/`: code generation scripts.
    -   `solidity/`:
        -   `inputs/`: Solidity language definition.
        -   `outputs/`: different packages and artifacts produced from it.
-   `documentation/`: mkdocs site to render project documentation.
-   `scripts/`: self-contained bash scripts to setup, test, run, and publish everything above.
