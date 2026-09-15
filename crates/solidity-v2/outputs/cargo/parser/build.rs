fn main() {
    // Without `emit_rerun_directives(true)`, lalrpop prints no `rerun-if-changed` at all,
    // and cargo falls back to watching every file in the package directory.
    // That means that regenerating unrelated files like `generated/public_api.txt`
    // forces a rebuild of this crate along with everything downstream of it.
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();
}
