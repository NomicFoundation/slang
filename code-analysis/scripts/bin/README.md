# Cargo Run Scripts

This folder contains scripts/shorthands for rust binaries we add to the Cargo workspace.
These scripts read and reuse the same configs used by other build/test scripts.

This folder is added to the hermit environment `$PATH`, it can be directly invoked from any folder. Example:

```bash
solidity_rust_cli --version 0.8.0 "./code-analysis/crates/solidity/samples/Bee_token.sol" --json "output.json"
```

Whenever we add new binaries (internal or external), we should add a new script to this folder:

```bash
cd "$REPO_ROOT/code-analysis/scripts/bin"
ln -s _slang_cargo_bin_runner.sh THE_NEW_BINARY_NAME
```
