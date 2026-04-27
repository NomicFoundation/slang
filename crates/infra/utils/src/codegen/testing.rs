use std::path::{Path, PathBuf};

use itertools::Itertools;
use serde::Serialize;

use crate::paths::PathExtensions;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SnapshotEntry {
    Test {
        name: String,
    },
    Group {
        name: String,
        children: Vec<SnapshotEntry>,
    },
}

pub fn collect_snapshot_tests(data_dir: &Path) -> Vec<SnapshotEntry> {
    match walk(data_dir) {
        Some(SnapshotEntry::Group { name: _, children }) => children,
        Some(SnapshotEntry::Test { .. }) => {
            panic!("Snapshots root must not be a test directory: {data_dir:?}")
        }
        None => Vec::new(),
    }
}

fn walk(parent_dir: &Path) -> Option<SnapshotEntry> {
    let mut has_input_sol = false;
    let mut has_generated_dir = false;
    let mut all_children = Vec::<PathBuf>::new();

    for child in std::fs::read_dir(parent_dir).unwrap() {
        let child = child.unwrap();
        let child_path = child.path();
        let child_type = child.file_type().unwrap();

        if child_type.is_dir() {
            match child_path.unwrap_name() {
                "generated" => {
                    has_generated_dir = true;
                }
                _ => {
                    all_children.push(child_path);
                }
            }
        } else if child_type.is_file() {
            match child_path.unwrap_name() {
                "input.sol" => {
                    has_input_sol = true;
                }
                ".gitattributes" => {
                    // Some tests have special line endings (CRLF/CR) stored as binary.
                }
                _ => {
                    panic!("Unexpected file in snapshot directory: {child_path:?}");
                }
            }
        } else {
            panic!("Unexpected entry in snapshot directory: {child_path:?}");
        }
    }

    if has_input_sol || has_generated_dir {
        assert!(
            has_input_sol,
            "Test directory is missing 'input.sol': {parent_dir:?}"
        );
        assert!(
            has_generated_dir,
            "Test directory is missing 'generated/' directory: {parent_dir:?}"
        );
        assert!(
            all_children.is_empty(),
            "Test directory must not contain other subdirectories: {parent_dir:?}"
        );

        return Some(SnapshotEntry::Test {
            name: parent_dir.unwrap_name().to_owned(),
        });
    }

    let children_entries: Vec<_> = all_children
        .iter()
        .sorted()
        .filter_map(|child_path| walk(child_path))
        .collect();

    if children_entries.is_empty() {
        None
    } else {
        Some(SnapshotEntry::Group {
            name: parent_dir.unwrap_name().to_owned(),
            children: children_entries,
        })
    }
}
