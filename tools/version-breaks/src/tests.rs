use crate::{api::analyze_test, builds::BuildInfo, utils::with_progress};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn collect_tests(tests_dir: PathBuf) -> Vec<PathBuf> {
    let mut tests = Vec::<PathBuf>::new();
    let mut ongoing = vec![tests_dir.join("tests")];

    while ongoing.len() > 0 {
        let current = ongoing.pop().unwrap();

        let possible_test = current.join("input.sol");
        if possible_test.exists() {
            tests.push(possible_test);
            continue;
        }

        let mut children = current
            .read_dir()
            .unwrap()
            .map(|child| child.unwrap())
            .filter(|child| child.file_type().unwrap().is_dir())
            .map(|child| child.path())
            .collect::<Vec<PathBuf>>();

        children.sort();
        ongoing.extend(children);
    }

    println!("\nFound {} tests:\n", tests.len());
    return tests;
}

pub fn execute_test(tests_dir: &PathBuf, test: &PathBuf, builds: &Vec<BuildInfo>) {
    let test_name = test.strip_prefix(&tests_dir).unwrap();

    let inputs = builds
        .iter()
        .map(|build| (Arc::new(test), build))
        .collect::<Vec<_>>();

    let mut outputs = with_progress(
        test_name.to_path_buf().to_str().unwrap(),
        &inputs,
        move |(test, build)| analyze_test(test, build),
    );

    outputs.sort_by(|a, b| a.version.cmp(&b.version));

    let mut previous_contents = String::from("");
    let test_output_dir = prepare_test_output_dir(test);

    let combined_file_path = test_output_dir.join("combined");
    let mut combined = vec![create_combined_header(
        &"📝".to_string(),
        "input.sol",
        "solidity",
        &test_name,
    )];

    outputs.iter().for_each(|output| {
        if output.contents.ne(&previous_contents) {
            let version = output.version.to_string();
            let output_path = test_output_dir.join(&version);
            std::fs::write(&output_path, &output.contents).unwrap();

            let snippet_path = &output_path.strip_prefix(&tests_dir).unwrap();
            combined.push(create_combined_header(
                &output.icon,
                &version,
                &output.language,
                snippet_path,
            ));

            previous_contents = output.contents.to_string();
        }
    });

    std::fs::write(&combined_file_path, combined.join("\n")).unwrap();
}

fn create_combined_header(icon: &String, title: &str, language: &str, file: &Path) -> String {
    return format!(
        "=== \"{} {}\"\n\n    ```{}\n    --8<-- {:?}\n    ```\n",
        icon, title, language, file
    );
}

fn prepare_test_output_dir(test: &PathBuf) -> PathBuf {
    let test_output_dir = test.parent().unwrap().join("output");
    if test_output_dir.exists() {
        std::fs::remove_dir_all(&test_output_dir).unwrap();
    }

    if !test_output_dir.exists() {
        std::fs::create_dir_all(&test_output_dir).unwrap();
    }

    return test_output_dir;
}
