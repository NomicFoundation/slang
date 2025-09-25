use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, bail, Result};
use semver::{BuildMetadata, Prerelease};
use serde::Deserialize;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use solidity_testing_utils::import_resolver::{ImportRemap, ImportResolver, SourceMap};
use solidity_testing_utils::{config, fetch};

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
        single_file_project.entrypoint = file.file;
        // Solar doesn't support easy crawling of imports like Slang does, so we remove all the files
        // that are not the entrypoint. That means that the files must be self-contained.
        single_file_project
            .sources
            .retain(|k, _| k == &single_file_project.entrypoint);
        map.insert(file.name, single_file_project);
    }

    Ok(map)
}

/// Structure for deserealizing the metadata from a json file.
#[derive(Deserialize)]
struct Metadata {
    pub sources: HashMap<String, Source>,
    pub compilation: Compilation,
}

#[derive(Deserialize)]
struct Source {
    pub content: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Compilation {
    pub compiler_version: String,
    pub fully_qualified_name: String,
    pub compiler_settings: CompilerSettings,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompilerSettings {
    pub remappings: Vec<String>,
}

pub struct SolidityProject {
    pub name: String,
    pub sources: HashMap<String, String>,
    pub entrypoint: String,
    pub compiler_version: String,
    pub import_resolver: ImportResolver,
}

impl TryFrom<Metadata> for SolidityProject {
    type Error = anyhow::Error;

    fn try_from(value: Metadata) -> std::result::Result<Self, Self::Error> {
        let sources: HashMap<String, String> = value
            .sources
            .into_iter()
            .map(|(k, v)| (k, v.content))
            .collect();
        let import_resolver =
            import_resolver_from(value.compilation.compiler_settings, sources.keys());
        let compiler_version = value.compilation.compiler_version;
        let idx = value
            .compilation
            .fully_qualified_name
            .rfind(':')
            .ok_or(anyhow!(
                "Not properly formatted qualified name {fq}",
                fq = value.compilation.fully_qualified_name
            ))?;
        let name = value.compilation.fully_qualified_name[idx + 1..].to_string();
        let entrypoint = value.compilation.fully_qualified_name[..idx].to_string();

        Ok(SolidityProject {
            name,
            sources,
            entrypoint,
            compiler_version,
            import_resolver,
        })
    }
}

fn import_resolver_from<'a, T: Iterator<Item = &'a String>>(
    compiler_settings: CompilerSettings,
    sources: T,
) -> ImportResolver {
    let import_remaps: Vec<ImportRemap> = compiler_settings
        .remappings
        .iter()
        .filter_map(|m| ImportRemap::new(m).ok())
        .filter(|remap| !remap.has_known_bug())
        .collect();

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
        let Ok(json_str) = fs::read_to_string(json_file) else {
            bail!("Error reading file {json_file:?}")
        };
        let Ok(metadata) = serde_json::from_str::<Metadata>(&json_str) else {
            bail!("Error parsing file {json_file:?}:\n{json_str}");
        };
        let sself: Self = Self::try_from(metadata)?;
        Ok(sself)
    }

    pub fn build_compilation_unit(&self) -> Result<CompilationUnit> {
        let mut version = semver::Version::parse(&self.compiler_version).unwrap();
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;

        let mut builder = CompilationBuilder::new(version, self)?;

        builder.add_file(&self.entrypoint)?;

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
