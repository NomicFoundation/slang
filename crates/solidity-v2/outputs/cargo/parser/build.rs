fn main() {
    lalrpop::Configuration::new()
        .emit_report(false)
        .log_verbose()
        .process_current_dir()
        .unwrap();
    // lalrpop::process_root().unwrap();
}
