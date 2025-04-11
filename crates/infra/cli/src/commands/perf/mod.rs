mod cargo;
mod npm;

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::blocking::get;
use serde_json::Value;

use crate::commands::perf::cargo::CargoController;
use crate::commands::perf::npm::NpmController;

#[derive(Clone, Debug, Parser)]
pub struct PerfController {
    #[command(subcommand)]
    command: PerfCommand,
}

#[derive(Clone, Debug, Subcommand)]
enum PerfCommand {
    /// Run benchmark tests for the rust api, and report the results to <https://bencher.dev/console>
    Cargo(CargoController),
    /// Run benchmark tests for the typescript api
    Npm(NpmController),
    /// Fetch the sources of contracts
    Fetch {
        /// Base path to store the fetched files
        path: String,
    },
}

impl PerfController {
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
        match &self.command {
            PerfCommand::Cargo(controller) => controller.execute(),
            PerfCommand::Npm(controller) => controller.execute(),
            PerfCommand::Fetch { path } => {
                let base_path = Path::new(path);
                // missing: 0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A
                Self::fetch("0x01a5E3268E3987f0EE5e6Eb12fe63fa2AF992D83", base_path)?;
                Self::fetch("0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b", base_path)?;
                Self::fetch("0x01abc00E86C7e258823b9a055Fd62cA6CF61a163", base_path)?;
                // missing: 0x01fef0d5d6fd6b5701ae913cafb11ddaee982c9a
                Self::fetch("0x015E220901014BAE4f7e168925CD74e725e23692", base_path)?;
                Self::fetch("0x0170f38fa8df1440521c8b8520BaAd0CdA132E82", base_path)?;
                Self::fetch("0x01665987bC6725070e56d160d75AA19d8B73273e", base_path)?;
                // missing: 0x8B3f33234ABD88493c0Cd28De33D583B70beDe35" (Lido)
                Self::fetch("0x1F98431c8aD98523631AE4a59f267346ea31F984", base_path)?;
                Self::fetch("0xcA11bde05977b3631167028862bE2a173976CA11", base_path)?;
                Self::fetch("0xba5Ed099633D3B313e4D5F7bdc1305d3c28ba5Ed", base_path)?;
                Ok(())
            }
        }
    }
}
