use std::fs;
use std::path::Path;

use anyhow::Error;
use clap::Parser;
use infra_utils::paths::PathExtensions;
use reqwest::blocking::get;
use serde_json::Value;

#[derive(Clone, Debug, Parser)]
pub struct FetchController {
    /// Base path to store the fetched files
    path: String,
    /// Hash when fetching a specific project
    #[arg(long)]
    hash: Option<String>,
}

impl FetchController {
    // Given an address and a path, it downloads the json file from sourcify,
    // and recreates the file system
    fn fetch(address: &str, base_path: &Path) -> Result<(), Error> {
        let url = format!(
            "https://sourcify.dev/server/v2/contract/1/{address}/?fields=sources,compilation"
        );
        let response = get(&url)?.text()?;
        let json: Value = serde_json::from_str(&response)?;

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
            // Ensure the path does not escape base_path
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

    pub fn execute(&self) -> Result<(), Error> {
        let base_path = Path::new(&self.path);

        if let Some(hash) = &self.hash {
            Self::fetch(hash, base_path)?;
        } else {
            // Read hashes from a configuration file
            let config_path = Path::repo_path("crates/infra/cli/src/commands/perf/projects.json");
            let config_content = fs::read_to_string(config_path)?;
            let config_json: serde_json::Value = serde_json::from_str(&config_content)?;

            let hashes = config_json
                .get("sourcifyHashes")
                .and_then(|h| h.as_array())
                .ok_or_else(|| {
                    anyhow::anyhow!("Invalid or missing 'sourcifyHashes' field in projects.json")
                })?;

            for hash in hashes {
                let project = hash
                    .as_object()
                    .ok_or_else(|| anyhow::anyhow!("Invalid project format in projects.json"))?;
                let hash_str = project
                    .get("hash")
                    .ok_or_else(|| anyhow::anyhow!("Invalid hash format in projects.json"))?
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid hash format in projects.json"))?;

                Self::fetch(hash_str, base_path)?;
            }
        }
        Ok(())
    }
}
