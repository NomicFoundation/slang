use crate::dataset::{SolidityProject, load_projects};

pub fn setup(project: &str) -> &'static SolidityProject {
    load_projects()
        .get(project)
        .unwrap_or_else(|| panic!("Can't find {project}"))
}

/// Downloads every benchmark input up front, so that no measured process ever does.
pub fn prefetch_inputs() {
    load_projects();
}
