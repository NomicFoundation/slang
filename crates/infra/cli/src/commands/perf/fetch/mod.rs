use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::config;
use reqwest::blocking::get;
use serde_json::Value;

#[derive(Clone, Debug, Parser)]
pub struct FetchController {
    /// Hash when fetching a specific project
    #[arg(long)]
    hash: Option<String>,
}

impl FetchController {
    // Given an address and a path, it downloads the json file from sourcify,
    // and recreates the file system as `<path>/<address>/`. We store in `./compilation.json`
    // the specifics of the compilation, in particular, `compilerVersion` and the `fullyQualifiedName`
    // (main entry point).
    fn fetch(address: &str, base_path: &Path) -> Result<()> {
        //TODO: generalize for any chain
        let url = format!(
            "https://sourcify.dev/server/v2/contract/1/{address}/?fields=sources,compilation"
        );
        let response = get(&url)?.text()?;
        let json: Value = serde_json::from_str(&response)?;

        // The expected json result should have two fields of interest to us: `sources`,
        // with the files, and `compilation`, with the specifics of compilation.

        let base_path = base_path.join(Path::new(address));

        let sources = json
            .get("sources")
            .and_then(|s| s.as_object())
            .ok_or_else(|| anyhow::anyhow!("Project {address} has no sources"))?;

        // Ensure the "compilation" field exists and save it as "compilation.json"
        let compilation = json
            .get("compilation")
            .ok_or_else(|| anyhow::anyhow!("Project {address} has no compilation field"))?;
        let compilation_path = base_path.join("compilation.json");
        let compilation_content = serde_json::to_string_pretty(compilation)?;
        fs::create_dir_all(&base_path)?;
        fs::write(compilation_path, compilation_content)?;

        for (path, file) in sources {
            let content = file
                .get("content")
                .and_then(|c| c.as_str())
                .ok_or_else(|| anyhow::anyhow!("File {path} of {address} has no content"))?;

            let path = Path::new(path);
            // Basic sanitizations: ensure the path does not escape base_path
            let sanitized_path = path.strip_prefix("/").unwrap_or(path);
            let file_path = base_path.join(sanitized_path);

            let parent = file_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Can't obtain parent dir for {file_path:?}"))?;
            fs::create_dir_all(parent)?;

            fs::write(file_path, content)?;
        }

        Ok(())
    }

    pub fn execute(&self) -> Result<()> {
        let config = config::read_config()?;

        let base_path = Path::new(&config.working_dir);

        if let Some(hash) = &self.hash {
            Self::fetch(hash, base_path)?;
        } else {
            // We need to download all the hashes, no matter where they come from.
            let mut hashes_projects: Vec<_> = config.projects.iter().map(|p| &p.hash).collect();
            let hashes_keys: Vec<_> = config.files.iter().map(|f| &f.hash).collect();

            hashes_projects.extend(hashes_keys);

            for hash in hashes_projects {
                Self::fetch(hash, base_path)?;
            }
        }
        Ok(())
    }
}
