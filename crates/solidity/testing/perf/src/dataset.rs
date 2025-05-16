use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::config::{self};
use infra_utils::paths::PathExtensions;

#[derive(Debug, Clone)]
pub struct SolidityProject {
    pub version: String,
    pub root: PathBuf,
    pub entrypoint: String,
}

impl SolidityProject {
    pub(crate) fn new(path: &Path) -> Result<SolidityProject> {
        let compilation_file = path.join("compilation.json");

        if !compilation_file.exists() {
            return Err(anyhow::anyhow!(
                "Missing compilation.json in folder: {:?}",
                path
            ));
        }

        let content = fs::read_to_string(&compilation_file)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;

        let fully_qualified_name = json
            .get("fullyQualifiedName")
            .and_then(|f| f.as_str())
            .ok_or_else(|| {
                anyhow::anyhow!("Missing fullyQualifiedName field in file: {compilation_file:?}")
            })?
            .rsplit_once(':')
            .map(|(before_last_colon, _)| before_last_colon)
            .ok_or_else(|| anyhow::anyhow!("fullyQualifiedName is not well formatted"))?;

        let compiler_version = json
            .get("compilerVersion")
            .and_then(|f| f.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing compilerVersion in {compilation_file:?}"))?;

        Ok(SolidityProject {
            version: compiler_version.to_owned(),
            root: path.to_path_buf(),
            entrypoint: fully_qualified_name.to_owned(),
        })
    }
}

pub fn load_projects() -> Result<Vec<SolidityProject>> {
    let config = config::read_config()?;
    let working_directory_path = Path::repo_path(&config.working_dir);
    let mut projects = vec![];

    for file in config.files {
        let mut project = SolidityProject::new(&working_directory_path.join(file.hash))?;
        // override the entrypoint with the path given
        project.entrypoint = file.file;
        projects.push(project);
    }

    for project in config.projects {
        let project = SolidityProject::new(&working_directory_path.join(project.hash))?;
        projects.push(project);
    }
    Ok(projects)
}
