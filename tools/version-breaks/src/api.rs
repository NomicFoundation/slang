use crate::{
    ast::{InputSource, OutputSettings, StandardInput, StandardOutput},
    builds::BuildInfo,
};
use semver::Version;
use std::{
    collections::HashMap,
    io::{Error, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct ApiResult {
    pub version: Version,
    pub contents: String,
    pub icon: String,
    pub language: String,
}

pub fn analyze_test(test: &PathBuf, build: &BuildInfo) -> ApiResult {
    // Sporadic broken pipe errors:
    // https://github.com/rust-lang/rust/issues/46016
    let retries = 5;
    let output = loop {
        match execute_standard_json_command(test, build) {
            Ok(output) => {
                break output;
            }
            Err(error) => {
                if --retries == 0 {
                    panic!("{:?}", error);
                }
            }
        }
    };

    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    let full_output: StandardOutput = serde_json::from_str(&stdout).expect(&stdout);

    match full_output.errors {
        None => {
            let ast = full_output
                .sources
                .unwrap()
                .get("input.sol")
                .unwrap()
                .ast
                .clone()
                .unwrap();

            return ApiResult {
                version: build.version.to_owned(),
                contents: serde_json::to_string_pretty(&ast).unwrap() + "\n",
                icon: "✅".to_string(),
                language: "json".to_string(),
            };
        }
        Some(errors) => {
            assert!(errors.len() > 0);

            let mut icon = "⚠️".to_string();
            let mut contents = String::new();

            errors.iter().for_each(|e| {
                match e.severity.as_str() {
                    "error" => icon = "❌".to_string(),
                    "warning" => { /* do nothing */ }
                    "info" => { /* do nothing */ }
                    _ => panic!("Unrecognized severity: {}", e.severity),
                }

                contents = format!("{}\n{}", contents, e.formatted_message);
            });

            return ApiResult {
                version: build.version.to_owned(),
                contents: contents.trim().to_owned(),
                icon: icon,
                language: "log".to_string(),
            };
        }
    }
}

fn execute_standard_json_command(
    test: &PathBuf,
    build: &BuildInfo,
) -> Result<std::process::Output, Error> {
    let test_contents = std::fs::read_to_string(&test).unwrap();

    let input = StandardInput {
        language: "Solidity".to_string(),
        sources: HashMap::from([(
            "input.sol".to_string(),
            InputSource {
                content: test_contents,
            },
        )]),
        settings: OutputSettings {
            output_selection: HashMap::from([(
                // for all files
                "*".to_string(),
                HashMap::from([(
                    // For the entire file
                    "".to_string(),
                    vec![
                        // generate the syntax tree
                        "ast".to_string(),
                    ],
                )]),
            )]),
        },
    };

    let input_string = serde_json::to_string(&input).unwrap();

    let mut command = Command::new(&build.local_path)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(format!("{:?} --standard-json {:?}", build.local_path, test).as_str());

    match command
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input_string.as_bytes())
    {
        Ok(()) => Ok(command.wait_with_output().unwrap()),
        Err(error) => Err(error),
    }
}
