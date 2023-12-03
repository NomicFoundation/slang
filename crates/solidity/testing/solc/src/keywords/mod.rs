use crate::utils::{ApiInput, Binary, InputSource};
use codegen_language_definition::model::{Item, KeywordDefinition, KeywordItem, Language};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use solidity_language::SolidityDefinition;
use std::collections::HashSet;

pub fn check_solidity_keywords() {
    println!();
    println!("  > Checking Solidity keywords:");
    println!();

    let language = SolidityDefinition::create();
    let test_cases = generate_test_cases(&language);

    println!();
    println!("  > Downloading solc binaries:");
    println!();

    let binaries = Binary::fetch_all(&language);

    println!();
    println!("  > Testing all variations:");
    println!();

    let progress_bar = ProgressBar::new(test_cases.len() as u64);

    let style = "[{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} │ ETA: {eta_precise}";
    progress_bar.set_style(ProgressStyle::with_template(style).unwrap());

    test_cases.par_iter().for_each(|test_case| {
        let result = test_case.execute(&binaries);

        if !result.should_be_reserved_in.is_empty() {
            progress_bar.println(format!(
                "[{item}] '{variation}' should be reserved in: {versions:?}",
                item = test_case.item,
                variation = test_case.variation,
                versions = result.should_be_reserved_in
            ));
        }

        if !result.should_be_unreserved_in.is_empty() {
            progress_bar.println(format!(
                "[{item}] '{variation}' should be unreserved in: {versions:?}",
                item = test_case.item,
                variation = test_case.variation,
                versions = result.should_be_unreserved_in
            ));
        }

        progress_bar.inc(1);
    });

    progress_bar.finish();
    println!();
}

fn generate_test_cases(language: &Language) -> Vec<TestCase> {
    let mut test_cases = vec![];
    let mut variations = HashSet::new();

    for section in &language.sections {
        for topic in &section.topics {
            for item in &topic.items {
                let Item::Keyword { item } = item.as_ref() else {
                    continue;
                };

                if !should_test_item(item.name.as_str()) {
                    continue;
                }

                for definition in &item.definitions {
                    for variation in definition.value.collect_variations() {
                        assert!(
                            variations.insert(format!("{} -> {}", item.identifier, variation)),
                            "Duplicate variation: {variation}"
                        );

                        test_cases.push(TestCase::new(language, item, definition, variation));
                    }
                }
            }
        }
    }

    test_cases
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
                "// SPDX-License-Identifier: GPL-3.0
                pragma solidity x.x.x;
                
                contract Foo {{
                    function bar() public {{
                        uint256 {variation} = 0;
                    }}
                }}"
            ),
            "YulIdentifier" => format!(
                "// SPDX-License-Identifier: GPL-3.0
                pragma solidity x.x.x;
                
                contract Foo {{
                    function bar() public {{
                        assembly {{
                            let {variation} := 1
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
            let success = self.test_version(binary);
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

    fn test_version(&self, binary: &Binary) -> bool {
        let input = ApiInput {
            language: "Solidity".into(),
            sources: [(
                "input.sol".into(),
                InputSource {
                    content: self.source.to_owned(),
                },
            )]
            .into(),
        };

        let output = match binary.run(&input) {
            Ok(output) => output,
            Err(error) => {
                let error = format!("{error:#?}");
                if binary.version == Version::new(0, 4, 11)
                    && error.contains("Command failed with code 'UNKNOWN' and signal '11'")
                {
                    // solc 0.4.11 SEGFAULTs when a keyword exists in an identifier position.
                    return false;
                }

                println!();
                println!(
                    "Invoking solc failed:\n{error}\n\nInput:\n{input}",
                    input = serde_json::to_string_pretty(&input).unwrap(),
                );
                std::process::exit(1);
            }
        };

        for error in output.errors.iter().flatten() {
            if [
                "Expected identifier, got",
                "Expected identifier but got",
                "Expected token Identifier got",
                "Expected token Semicolon got",
                "Expected ';' but got ",
                "State mutability can only be specified for address types.",
                "Cannot use instruction names for identifier names.",
                "Cannot use builtin function name",
            ]
            .iter()
            .any(|part| error.message.contains(part))
            {
                // Identifier parsed as a keyword.
                return false;
            }

            if [
                "Unused local variable",
                "Function state mutability can be restricted to pure",
                "This declaration shadows a builtin symbol.",
                "This declaration shadows a declaration outside",
            ]
            .iter()
            .any(|part| error.message.contains(part))
            {
                // Unrelated
                continue;
            }

            println!("Unexpected error: {error:#?}");
            std::process::exit(1);
        }

        true
    }
}

fn should_test_item(item: &str) -> bool {
    match item {
        "FixedKeyword" | "UfixedKeyword" | "YulUfixedKeyword" | "YulFixedKeyword" => {
            println!("WARNING: skipping '{item}' by default, as it generates thousands of variations. Enable manually if needed.");
            false
        }
        _ => true,
    }
}
