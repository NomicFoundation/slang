use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use semver::Version;

pub trait GrammarTestingExtensions {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        snapshots_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()>;
}

impl GrammarTestingExtensions for Grammar {
    fn generate_cst_output_tests(
        &self,
        codegen: &mut CodegenContext,
        snapshots_dir: &PathBuf,
        output_dir: &PathBuf,
    ) -> Result<()> {
        codegen.rerun_if_changed(snapshots_dir)?;

        let breaking_versions = &self.get_breaking_versions();
        let parser_names = &collect_parser_names(snapshots_dir)?;

        generate_mod_file(
            codegen,
            parser_names,
            breaking_versions,
            &output_dir.join("mod.rs"),
        )?;

        for parser_name in parser_names {
            generate_unit_test_file(
                codegen,
                parser_name,
                &snapshots_dir.join(parser_name),
                &output_dir.join(format!("{parser_name}.rs")),
            )?;
        }

        return Ok(());
    }
}

fn collect_parser_names(snapshots_dir: &PathBuf) -> Result<Vec<String>> {
    let mut parser_names = snapshots_dir.read_dir()?.map(|entry| {
        let entry = entry?;

        // TODO(OmarTawfik): check any left-over/extra entries

        if !entry.file_type()?.is_dir() {
            bail!("Snapshots root directory should contain only sub-directories for each parser: {entry:?}");
        }

        let parser_name = entry
            .file_name()
            .to_str()
            .context(format!("Failed to get parser name from entry: {entry:?}"))?
            .to_owned();

        return Ok(parser_name);
    }).collect::<Result<Vec<String>>>()?;

    parser_names.sort();

    return Ok(parser_names);
}

fn generate_mod_file(
    codegen: &mut CodegenContext,
    parser_names: &Vec<String>,
    breaking_versions: &Vec<&Version>,
    mod_file_path: &PathBuf,
) -> Result<()> {
    let contents = format!(
        "
            {module_declarations}

            use std::rc::Rc;

            use anyhow::{{bail, Result}};
            use solidity_rust_lib::generated::{{
                cst,
                parse::{{ParserType, Parsers}},
            }};

            pub const BREAKING_VERSIONS: [&str; {breaking_versions_size}] = [
                {breaking_versions}
            ];

            pub fn get_parser<'a>(
                parsers: Parsers<'a>,
                parser_name: &str,
            ) -> Result<ParserType<'a, Rc<cst::Node>>> {{
                return Ok(match parser_name {{
                    {parser_name_match_arms}
                    _ => bail!(\"Unrecognized parser_name: {{parser_name}}\"),
                }});
            }}
        ",
        module_declarations = parser_names
            .iter()
            .map(|parser_name| format!("mod {parser_name};"))
            .collect::<String>(),
        breaking_versions_size = breaking_versions.len(),
        breaking_versions = breaking_versions
            .iter()
            .map(|version| format!("\"{version}\","))
            .collect::<String>(),
        parser_name_match_arms = parser_names
            .iter()
            .map(|parser_name| format!("\"{parser_name}\" => parsers.{parser_name},"))
            .collect::<String>(),
    );

    return codegen.write_file(mod_file_path, &contents);
}

fn generate_unit_test_file(
    codegen: &mut CodegenContext,
    parser_name: &str,
    parser_snapshots_dir: &PathBuf,
    unit_test_file_path: &PathBuf,
) -> Result<()> {
    let test_names = collect_test_names(parser_snapshots_dir)?;

    let contents = format!(
        "
            use crate::cst_output::run;
            use anyhow::Result;

            {unit_tests}
        ",
        unit_tests = test_names
            .iter()
            .map(|test_name| format!(
                "
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        return run(\"{parser_name}\", \"{test_name}\");
                    }}
                "
            ))
            .collect::<String>(),
    );

    return codegen.write_file(unit_test_file_path, &contents);
}

fn collect_test_names(parser_snapshots_dir: &PathBuf) -> Result<Vec<String>> {
    let mut test_names = parser_snapshots_dir.read_dir()?.map(|entry| {
        let entry = entry?;

        // TODO(OmarTawfik): check any left-over/extra entries

        if !entry.file_type()?.is_dir() {
            bail!("Parser snapshots directory should contain only sub-directories for each test: {entry:?}");
        }

        let test_name = entry
            .file_name()
            .to_str()
            .context(format!("Failed to get test name from entry: {entry:?}"))?
            .to_owned();

        return Ok(test_name);
    }).collect::<Result<Vec<String>>>()?;

    test_names.sort();

    return Ok(test_names);
}
