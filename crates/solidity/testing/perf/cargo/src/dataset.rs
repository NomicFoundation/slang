use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use semver::{BuildMetadata, Prerelease};
use serde::Deserialize;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use solidity_testing_perf_utils::import_resolver::{ImportRemap, ImportResolver, SourceMap};
use solidity_testing_perf_utils::{config, fetch};

type ProjectMap = HashMap<String, SolidityProject>;

pub fn load_projects() -> &'static ProjectMap {
    static CACHE: OnceLock<ProjectMap> = OnceLock::new();

    CACHE.get_or_init(|| load_projects_internal().unwrap())
}

fn load_projects_internal() -> Result<ProjectMap> {
    let mut map = ProjectMap::new();
    let config = config::read_config()?;
    let working_directory_path = config::working_dir_path();

    for project in config.projects {
        fetch::fetch(&project.hash, &working_directory_path)?;

        let sol_project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", project.hash)))?;
        map.insert(project.name, sol_project);
    }

    for file in config.files {
        fetch::fetch(&file.hash, &working_directory_path)?;

        let mut single_file_project =
            SolidityProject::build(&working_directory_path.join(format!("{}.json", file.hash)))?;
        // override the entrypoint with the path given
        single_file_project.compilation.set_entrypoint(&file.file);
        // Solar doesn't support easy crawling of imports like Slang does, so we remove all the files
        // that are not the entrypoint. That means that the files must be self-contained.
        single_file_project.sources.retain(|k, _| k == &file.file);
        map.insert(file.name, single_file_project);
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

pub struct SolidityProject {
    pub sources: HashMap<String, String>,
    pub compilation: SolidityCompilation,
    pub import_resolver: ImportResolver,
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

impl TryFrom<&serde_json::Value> for SolidityProject {
    type Error = anyhow::Error;

    fn try_from(value: &serde_json::Value) -> std::result::Result<Self, Self::Error> {
        let sources =
            sources_map_deserializer(value.get("sources").expect("json must have 'sources' key"))?;
        let compilation: SolidityCompilation = serde_json::from_value(
            value
                .get("compilation")
                .expect("json must have 'compilation' key")
                .to_owned(),
        )?;
        let import_resolver = import_resolver_from(value, sources.keys());
        Ok(SolidityProject {
            sources,
            compilation,
            import_resolver,
        })
    }
}

fn import_resolver_from<'a, T: Iterator<Item = &'a String>>(
    value: &serde_json::Value,
    sources: T,
) -> ImportResolver {
    let import_remaps: Vec<ImportRemap> = value
        .get("compilation")
        .expect("json must have a 'compiler' key")
        .get("compilerSettings")
        .and_then(|settings| settings.get("remappings"))
        .and_then(|remappings| remappings.as_array())
        .map_or(vec![], |mappings| {
            mappings
                .iter()
                .filter_map(|mapping| mapping.as_str())
                .filter_map(|m| ImportRemap::new(m).ok())
                .filter(|remap| !remap.has_known_bug())
                .collect()
        });

    let source_maps: Vec<SourceMap> = sources
        .map(|path| SourceMap {
            source_id: path.clone(),
            virtual_path: path.clone(),
        })
        .collect();

    ImportResolver {
        import_remaps,
        source_maps,
    }
}

impl SolidityProject {
    pub fn build(json_file: &Path) -> Result<Self> {
        let json_str = fs::read_to_string(json_file)?;
        let json_value: serde_json::Value = serde_json::from_str(&json_str)?;
        let sself: Self = Self::try_from(&json_value)?;
        Ok(sself)
    }

    pub fn build_compilation_unit(&self) -> Result<CompilationUnit> {
        let mut version = semver::Version::parse(&self.compilation.compiler_version).unwrap();
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;

        let mut builder = CompilationBuilder::new(version, self)?;

        builder.add_file(&self.compilation.get_entrypoint())?;

        Ok(builder.build())
    }
}

impl CompilationBuilderConfig for &SolidityProject {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        self.sources
            .get(file_id)
            .map(|s| Some(s.to_owned()))
            .ok_or(anyhow!("Can't find {}", file_id))
    }

    /// Resolves an import of a solidity file.
    /// - `source_file`: the relative path to the file under inspection,
    /// - `import_string`: the import string as parsed from the source file.
    ///
    /// Returns the relative path of the imported file.
    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        let import_path = import_path_cursor.node().unparse();
        let import_path = import_path
            .strip_prefix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .strip_suffix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .trim();

        self.import_resolver
            .resolve_import(source_file_id, import_path)
            .ok_or_else(|| {
                anyhow!(
                    "Can't resolve import '{}' from '{}'",
                    import_path,
                    source_file_id
                )
            })
            .map(Some)
    }
}
