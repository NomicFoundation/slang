use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use heck::ToSnakeCase;
use solidity_testing_perf_utils::config;

fn main() -> Result<()> {
    println!("cargo::rerun-if-changed={:?}", config::config_file_path()?);
    println!("cargo::rerun-if-changed=build.rs");

    let cfg = config::read_config()?;

    let mut test_benches_file = File::create("src/benches_list.rs")?;

    let mut write_test_benches_line = |str: &String| {
        writeln!(
            test_benches_file,
            "    define_payload_tests!({}, \"{}\");",
            str.to_snake_case(),
            str
        )
    };

    for project in cfg.projects {
        write_test_benches_line(&project.name)?;
    }
    for pfile in cfg.files {
        let name = Path::new(&pfile.file)
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        write_test_benches_line(&name)?;
    }

    Ok(())
}
