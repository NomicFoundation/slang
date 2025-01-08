use std::fs;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use semver::Version;
use slang_solidity::parser::Parser;
use slang_solidity::{bindings, diagnostic};

use crate::bindings::lookup_definition_by_name;
use crate::bindings_assertions::assertions::{
    check_assertions, collect_assertions_into, Assertions,
};
use crate::generated::VERSION_BREAKS;
use crate::multi_part_file::{split_multi_file, Part};
use crate::resolver::TestsPathResolver;

pub fn run(group_name: &str, test_name: &str) -> Result<()> {
    let file_name = format!("{test_name}.sol");
    let data_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_assertions")
        .join(group_name);
    let input_path = data_dir.join(file_name);
    let contents = fs::read_to_string(input_path)?;

    for version in &VERSION_BREAKS {
        check_assertions_with_version(version, &contents)?;
    }
    Ok(())
}

fn check_assertions_with_version(version: &Version, contents: &str) -> Result<()> {
    let parser = Parser::create(version.clone())?;
    let mut binding_graph =
        bindings::create_with_resolver(version.clone(), Rc::new(TestsPathResolver {}))?;

    let mut assertions = Assertions::new();
    let mut skipped = 0;

    let multi_part = split_multi_file(contents);

    for Part {
        name: file_path,
        contents: file_contents,
    } in &multi_part.parts
    {
        let parse_output = parser.parse(Parser::ROOT_KIND, file_contents);

        if !parse_output.is_valid() {
            let report = parse_output
                .errors()
                .iter()
                .map(|error| diagnostic::render(error, file_path, file_contents, false))
                .collect::<Vec<_>>()
                .join("\n");
            eprintln!("\nParse errors for version {version}\nFile: {file_path}\n{report}");
        }

        binding_graph.add_user_file(file_path, parse_output.create_tree_cursor());
        skipped += collect_assertions_into(
            &mut assertions,
            parse_output.create_tree_cursor(),
            file_path,
            version,
        )?;
    }

    if let Some(context) = multi_part.context {
        let context_definition = lookup_definition_by_name(&binding_graph, context)
            .expect("context definition to be found")
            .to_handle();
        binding_graph.set_context(&context_definition);
    }

    let result = check_assertions(&binding_graph, &assertions, version);

    match result {
        Ok(count) => {
            assert!(count > 0, "No assertions found with version {version}");
            println!("Version {version}, {count} assertions OK, {skipped} skipped");
        }
        Err(err) => {
            panic!("Failed bindings assertions in version {version}:\n{err}");
        }
    }

    Ok(())
}
