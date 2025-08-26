use solidity_testing_perf_utils::config::Library;

use crate::dataset::{load_projects, SolidityProject};

pub fn setup(project: &str, lib: Library) -> Option<&'static SolidityProject> {
    let project_with_exclusion = load_projects().get(project).unwrap();
    if project_with_exclusion
        .exclude
        .as_ref()
        .is_some_and(|exclude| !exclude.contains(&lib))
    {
        None
    } else {
        Some(&project_with_exclusion.project)
    }
}
