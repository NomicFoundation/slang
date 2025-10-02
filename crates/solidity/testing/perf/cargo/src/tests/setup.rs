use crate::dataset::{load_projects, SolidityProject};

pub fn setup(project: &str) -> &'static SolidityProject {
    load_projects()
        .get(project)
        .unwrap_or_else(|| panic!("Can't find {project}"))
}
