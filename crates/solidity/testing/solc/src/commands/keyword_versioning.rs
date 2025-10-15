use std::collections::HashSet;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use infra_utils::terminal::NumbersDefaultDisplay;
use language_definition::model::{Item, KeywordDefinition, KeywordItem, Language};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use semver::Version;
use solidity_language::SolidityDefinition;

use crate::utils::{Binary, CliInput, InputSource, LanguageSelector};

/// Makes sure all Solidity definition keywords have the corrent version specifiers.
#[derive(Debug, Parser)]
pub struct KeywordVersioningCommand {
    /// By default, all keyword variations are tested:
    /// Most keywords generate only one variation (`function`, `contract`, `struct`, ...).
    /// A few keywords generate 2-50 variations (`bytes1`, `bytes2`, `bytes3`, ...).
    /// And a handful generate thousands of variations (`fixed{N}x{M}`, `ufixed{N}x{M}`, ...).
    ///
    /// For local development, you can adjust the limit as needed.
    /// For example, using a limit of 10 will test the majority of the grammar, but in a fraction of the time.
    #[arg(long)]
    variations_limit: Option<usize>,
}

impl KeywordVersioningCommand {
    pub fn execute(self) -> Result<()> {
        println!();
        println!("  > Checking Solidity keywords:");
        println!();

        let language = SolidityDefinition::create();
        let test_cases = self.generate_test_cases(&language);

        println!();
        println!("  > Downloading solc binaries:");
        println!();

        let binaries = Binary::fetch_all(&language)?;

        println!();
        println!("  > Testing all variations:");
        println!();

        let progress_bar = ProgressBar::new(test_cases.len() as u64);

        let style = "[{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} │ ETA: {eta_precise}";
        progress_bar.set_style(ProgressStyle::with_template(style)?);

        let total_errors = test_cases
            .par_iter()
            .map(|test_case| {
                let result = test_case.execute(&binaries);
                let mut errors = 0;

                if !result.should_be_reserved_in.is_empty() {
                    errors += 1;
                    progress_bar.println(format!(
                        "[{item}] '{variation}' should be reserved in: {versions:?}",
                        item = test_case.item,
                        variation = test_case.variation,
                        versions = result.should_be_reserved_in
                    ));
                }

                if !result.should_be_unreserved_in.is_empty() {
                    errors += 1;
                    progress_bar.println(format!(
                        "[{item}] '{variation}' should be unreserved in: {versions:?}",
                        item = test_case.item,
                        variation = test_case.variation,
                        versions = result.should_be_unreserved_in
                    ));
                }

                progress_bar.inc(1);
                errors
            })
            .sum::<usize>();

        progress_bar.finish();
        println!();

        println!();
        println!("Found {total_errors} errors in keyword definitions.");
        println!();

        if total_errors > 0 {
            std::process::exit(1);
        }

        Ok(())
    }

    fn generate_test_cases(&self, language: &Language) -> Vec<TestCase> {
        let mut test_cases = vec![];
        let mut already_seen = HashSet::new();

        for item in language.items() {
            let Item::Keyword { item } = item else {
                continue;
            };

            // These are considered issues with earlier versions of 'solc', and we should skip from checking here:
            match item.name.as_str() {
                "AddressKeyword" | "YulAddressKeyword" => {
                    // Solc doesn't consistently reserve 'address' in all identifier positions.
                    // Sometimes it is reserved as a variable name, but allowed as a field name.
                    // Slang considers it unreserved for correctness. Let's skip it from testing here.
                    // https://github.com/NomicFoundation/slang/pull/1134
                    continue;
                }
                _ => {}
            }

            for definition in &item.definitions {
                let mut variations = definition.value.collect_variations();

                match self.variations_limit {
                    Some(limit) => {
                        if variations.len() > limit {
                            println!(
                                "WARNING: One of '{name}' definitions generated {total} variations. Based on the '--variations-limit' option provided, only testing the first {limit} variations (skipping {skipped}).",
                                name = item.name,
                                total = variations.len().num_display(),
                                skipped = (variations.len() - limit).num_display(),
                            );

                            variations.truncate(limit);
                        }
                    }
                    None => {
                        if variations.len() > 100 {
                            // Hint to the user that they can limit the variations:
                            println!(
                                "INFO: One of '{name}' definitions generated {total} variations. Testing all of them. You can limit the variations using the '--variations-limit' option.",
                                name = item.name,
                                total = variations.len().num_display(),
                            );
                        }
                    }
                }

                for variation in variations {
                    assert!(
                        already_seen.insert((&item.identifier, variation.clone())),
                        "Duplicate variation: {variation}"
                    );

                    test_cases.push(TestCase::new(language, item, definition, variation));
                }
            }
        }

        test_cases
    }
}

struct TestCase {
    item: String,
    variation: String,
    reserved_in: HashSet<Version>,

    source: String,
}

struct TestResult {
    should_be_reserved_in: Vec<String>,
    should_be_unreserved_in: Vec<String>,
}

impl TestCase {
    fn new(
        language: &Language,
        item: &KeywordItem,
        definition: &KeywordDefinition,
        variation: String,
    ) -> Self {
        let source = match item.identifier.as_str() {
            "Identifier" => format!(
                "// SPDX-License-Identifier: None
                pragma solidity x.x.x;
                
                contract Foo {{
                    function bar() public {{
                        uint256 {variation} = 0;
                    }}
                }}"
            ),
            "YulIdentifier" => format!(
                "// SPDX-License-Identifier: None
                pragma solidity x.x.x;
                
                contract Foo {{
                    function bar() public {{
                        assembly {{
                            let {variation} := 0
                        }}
                    }}
                }}"
            ),
            other => {
                panic!("Unexpected identifier: {other}");
            }
        };

        let reserved_in = language
            .versions
            .iter()
            .filter(|version| match &definition.reserved {
                None => true,
                Some(specifier) => specifier.contains(version),
            })
            .cloned()
            .collect();

        Self {
            item: item.name.to_string(),
            variation,
            reserved_in,

            source,
        }
    }

    fn execute(&self, binaries: &Vec<Binary>) -> TestResult {
        let mut should_be_reserved_in = vec![];
        let mut should_be_unreserved_in = vec![];

        for binary in binaries {
            let success = self.test_version(binary).unwrap();
            let is_reserved = self.reserved_in.contains(&binary.version);

            if success != is_reserved {
                // Language definition is correct.
                continue;
            }

            if is_reserved {
                should_be_unreserved_in.push(binary.version.to_string());
            } else {
                should_be_reserved_in.push(binary.version.to_string());
            }
        }

        TestResult {
            should_be_reserved_in,
            should_be_unreserved_in,
        }
    }

    fn test_version(&self, binary: &Binary) -> Result<bool> {
        let input = CliInput {
            language: LanguageSelector::Solidity,
            sources: [(
                "input.sol".into(),
                InputSource {
                    content: self.source.clone(),
                },
            )]
            .into(),
        };

        let output = match binary.run(&input) {
            Ok(output) => output,
            Err(error) => {
                println!();
                println!(
                    "Invoking solc failed:\n{error:#?}\n\nInput:\n{input}",
                    input = serde_json::to_string_pretty(&input)?,
                );
                std::process::exit(1);
            }
        };

        let actual_errors = &[
            // Identifier parsed as a keyword:
            Regex::new(r#"^Cannot use builtin function name ".+" as identifier name\.$"#)?,
            Regex::new(r#"^Cannot use instruction name ".+" as identifier name\.$"#)?,
            Regex::new(r#"^Cannot use instruction names for identifier names\.$"#)?,
            Regex::new(r#"^Expected '.+' but got '.+'$"#)?,
            Regex::new(r#"^Expected '.+' but got reserved keyword '.+'$"#)?,
            Regex::new(r#"^Expected \w+ but got '.+'$"#)?,
            Regex::new(r#"^Expected \w+ but got reserved keyword '.+'$"#)?,
            Regex::new(r#"^Expected \w+, got '.+'$"#)?,
            Regex::new(r#"^Expected token \w+ got '.+'$"#)?,
            Regex::new(r#"^Expected token \w+ got reserved keyword '.+'$"#)?,
            Regex::new(r#"^The identifier ".+" is reserved and can not be used\.$"#)?,
            Regex::new(r#"^The identifier name ".+" is reserved\.$"#)?,
            Regex::new(r#"^The name ".+" is reserved\.$"#)?,
        ];

        let ignored_errors = &[
            // Unrelated to keyword versioning:
            Regex::new(r#"^Function state mutability can be restricted to pure$"#)?,
            Regex::new(r#"^State mutability can only be specified for address types\.$"#)?,
            Regex::new(r#"^This declaration shadows a builtin symbol\.$"#)?,
            Regex::new(
                r#"^This declaration shadows a declaration outside the inline assembly block\.$"#,
            )?,
            Regex::new(r#"^Unused local variable\.?$"#)?,
        ];

        for error in output.errors.iter().flatten() {
            if actual_errors
                .iter()
                .any(|pattern| pattern.is_match(&error.message))
            {
                return Ok(false);
            }

            if ignored_errors
                .iter()
                .any(|pattern| pattern.is_match(&error.message))
            {
                continue;
            }

            println!("Unexpected error: {error:#?}");
            std::process::exit(1);
        }

        Ok(true)
    }
}
