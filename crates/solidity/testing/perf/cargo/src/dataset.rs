use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use infra_utils::paths::PathExtensions;
use solidity_testing_perf_utils::config;

type ProjectMap = HashMap<String, SolidityProject>;

pub struct SolidityCompilation {
    pub compiler_version: String,
    pub entrypoint: String,
    pub project_name: String,
}

impl SolidityCompilation {
    /// `fully_qualified_name` format is that of sourcify: `entrypoint:project`
    pub fn new(compiler_version: &str, fully_qualified_name: &str) -> Self {
        let idx = fully_qualified_name.rfind(':').unwrap();

        Self {
            compiler_version: compiler_version.to_string(),
            entrypoint: fully_qualified_name[..idx].to_string(),
            project_name: fully_qualified_name[idx + 1..].to_string(),
        }
    }

    pub fn set_entrypoint(&mut self, new_entry_point: &str) {
        self.entrypoint = new_entry_point.to_string();
    }
}

pub struct SolidityProject {
    pub sources: HashMap<String, String>,
    pub compilation: SolidityCompilation,
}

impl SolidityProject {
    fn new(sources: HashMap<String, String>, compilation: SolidityCompilation) -> Self {
        Self {
            sources,
            compilation,
        }
    }

    pub fn build(json_file: &Path) -> Result<Self> {
        let json_str = fs::read_to_string(json_file)?;
        let json: serde_json::Value = serde_json::from_str(&json_str)?;

        let mut sources = HashMap::new();
        if let Some(srcs) = json.get("sources").and_then(|v| v.as_object()) {
            for (file, data) in srcs {
                if let Some(content) = data.get("content") {
                    sources.insert(file.clone(), content.as_str().unwrap().to_owned());
                } else {
                    return Err(anyhow!("Invalid source in json"));
                }
            }
        } else {
            return Err(anyhow!("No sources in json"));
        }

        let compilation_json = json
            .get("compilation")
            .ok_or(anyhow!("No compilation data in json"))?;

        let compiler_version = compilation_json
            .get("compilerVersion")
            .and_then(|v| v.as_str())
            .ok_or(anyhow!("No proper version in json"))?;

        let fully_qualified_name = compilation_json
            .get("fullyQualifiedName")
            .and_then(|v| v.as_str())
            .ok_or(anyhow!("No proper fullyQualifiedName in json"))?;

        let compilation = SolidityCompilation::new(compiler_version, fully_qualified_name);

        Ok(SolidityProject::new(sources, compilation))
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
        let file = normalize_path(source_file_dir.join(import_string));

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

fn load_projects_internal() -> Result<ProjectMap> {
    let mut map = ProjectMap::new();
    let config = config::read_config()?;
    let working_directory_path = Path::repo_path(config::WORKING_DIR);

    let mut insert = |key: String, project: SolidityProject| {
        map.insert(key, project);
    };

    for file in config.files {
        let mut project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", file.hash)))?;
        // override the entrypoint with the path given
        project.compilation.set_entrypoint(&file.file);
        insert(file.name, project);
    }

    for project in config.projects {
        let sol_project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", project.hash)))?;
        insert(project.name, sol_project);
    }
    Ok(map)
}

pub fn load_projects() -> &'static ProjectMap {
    static CACHE: OnceLock<ProjectMap> = OnceLock::new();

    CACHE.get_or_init(|| load_projects_internal().unwrap())
}
