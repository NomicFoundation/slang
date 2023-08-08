// Need a fake library here so that `solidity_npm_crate` can depend on it during the build.
//
// Below are dependencies used by the CLI "main.rs", but not the API `lib.rs`.
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
mod supress_cli_dependencies {
    use anyhow as _;
    use codegen_schema as _;
    use codegen_syntax as _;
    use infra_utils as _;
    use solidity_language as _;
}
