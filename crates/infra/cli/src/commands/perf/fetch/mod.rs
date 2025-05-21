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
        let project_json: Value = get(&url)?.json()?;

        let file_name = address.to_owned() + ".json";
        let project_file_path = base_path.join(Path::new(&file_name));

        let content = serde_json::to_string_pretty(&project_json)?;
        fs::create_dir_all(base_path)?;
        fs::write(project_file_path, content)?;

        Ok(())
    }

    pub fn execute(&self) -> Result<()> {
        let config = config::read_config()?;

        let base_path = Path::new(&config::WORKING_DIR);

        if let Some(hash) = &self.hash {
            Self::fetch(hash, base_path)?;
        } else {
            // We need to download all the hashes, no matter where they come from.
            let hashes_projects = config
                .projects
                .iter()
                .map(|p| &p.hash)
                .chain(config.files.iter().map(|f| &f.hash));

            for hash in hashes_projects {
                Self::fetch(hash, base_path)?;
            }
        }
        Ok(())
    }
}
