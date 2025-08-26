use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use solidity_testing_perf_utils::config::Library;
use solidity_testing_perf_utils::{config, fetch};

type ProjectMap = HashMap<String, ProjectWithExclusion>;

pub struct ProjectWithExclusion {
    pub project: SolidityProject,
    pub exclude: Option<Vec<Library>>,
}

impl ProjectWithExclusion {
    pub(crate) fn from_project(project: SolidityProject) -> Self {
        ProjectWithExclusion {
            project,
            exclude: None,
        }
    }
}

impl From<SolidityProject> for ProjectWithExclusion {
    fn from(val: SolidityProject) -> Self {
        ProjectWithExclusion::from_project(val)
    }
}

pub fn load_projects() -> &'static ProjectMap {
    static CACHE: OnceLock<ProjectMap> = OnceLock::new();

    CACHE.get_or_init(|| load_projects_internal().unwrap())
}

fn load_projects_internal() -> Result<ProjectMap> {
    let mut map = ProjectMap::new();
    let config = config::read_config()?;
    let working_directory_path = config::working_dir_path();

    for file in config.files {
        fetch::fetch(&file.hash, &working_directory_path)?;

        let mut project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", file.hash)))?;
        // override the entrypoint with the path given
        project.compilation.set_entrypoint(&file.file);
        // Solar doesn't support easy crawling of imports like Slang does, so we remove all the files
        // that are not the entrypoint. That means that the files must be self-contained.
        project.sources.retain(|k, _| k == &file.file);
        map.insert(file.name, project.into());
    }

    for project in config.projects {
        fetch::fetch(&project.hash, &working_directory_path)?;

        let sol_project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", project.hash)))?;
        map.insert(
            project.name,
            ProjectWithExclusion {
                project: sol_project,
                exclude: project.exclude,
            },
        );
    }
    Ok(map)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidityCompilation {
    pub compiler_version: String,
    pub fully_qualified_name: String,
}

impl SolidityCompilation {
    pub fn get_entrypoint(&self) -> String {
        let idx = self.fully_qualified_name.rfind(':').unwrap();
        self.fully_qualified_name[..idx].to_string()
    }

    pub fn project_name(&self) -> String {
        let idx = self.fully_qualified_name.rfind(':').unwrap();
        self.fully_qualified_name[idx + 1..].to_string()
    }

    pub fn set_entrypoint(&mut self, new_entry_point: &str) {
        let pn = self.project_name();

        self.fully_qualified_name = format!("{new_entry_point}:{pn}");
    }
}

#[derive(Deserialize)]
pub struct SolidityProject {
    #[serde(deserialize_with = "sources_map_deserializer")]
    pub sources: HashMap<String, String>,
    pub compilation: SolidityCompilation,
}

fn sources_map_deserializer<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();
    for (file, value) in raw {
        let content = value
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                serde::de::Error::custom(format!("Missing or invalid content for file {file}"))
            })?;
        result.insert(file, content.to_string());
    }
    Ok(result)
}

impl SolidityProject {
    pub fn build(json_file: &Path) -> Result<Self> {
        let json_str = fs::read_to_string(json_file)?;
        let sself: Self = serde_json::from_str(&json_str)?;
        Ok(sself)
    }

    pub fn file_contents(&self, file: &str) -> Result<&str> {
        self.sources
            .get(file)
            .map(|s| s.as_str())
            .ok_or(anyhow!("Can't find {}", file))
    }

    /// Resolves an import of a solidity file.
    /// - `source_file`: the relative path to the file under inspection,
    /// - `import_string`: the import string as parsed from the source file.
    ///
    /// Returns the relative path of the imported file.
    pub fn resolve_import(&self, source_file: &str, import_string: &str) -> Result<String> {
        let source_file_dir = Path::new(source_file).parent().unwrap();
        let file = Self::normalize_path(source_file_dir.join(import_string));

        if self.sources.contains_key(&file) {
            Ok(file)
        } else if self.sources.contains_key(import_string) {
            Ok(import_string.to_string())
        } else {
            Err(anyhow!(
                "Can't resolve import {} ({} in the context of {:?})",
                file,
                import_string,
                source_file_dir
            ))
        }
    }

    fn normalize_path<P: AsRef<Path>>(path: P) -> String {
        let mut components = Vec::new();
        for comp in path.as_ref().components() {
            match comp {
                Component::ParentDir => {
                    components.pop();
                }
                Component::CurDir => {}
                other => components.push(other.as_os_str()),
            }
        }
        let mut normalized = PathBuf::new();
        for c in &components {
            normalized = normalized.join(c);
        }
        normalized.to_string_lossy().into_owned()
    }
}
