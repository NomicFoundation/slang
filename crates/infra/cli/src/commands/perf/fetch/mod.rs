use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::config;
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
    fn fetch(address: &str, base_path: &Path) -> Result<()> {
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

    pub fn execute(&self) -> Result<()> {
        let base_path = Path::new(&self.path);

        if let Some(hash) = &self.hash {
            Self::fetch(hash, base_path)?;
        } else {
            let config = config::read_config()?;
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
