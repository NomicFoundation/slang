use std::fs;
use std::path::Path;

use anyhow::{Ok, Result};
use reqwest::blocking::get;
use serde_json::Value;

// Given an address and a path, it downloads the json file from sourcify,
// and stores it in the path.
pub fn fetch(address: &str, base_path: &Path) -> Result<()> {
    let file_name = address.to_owned() + ".json";
    let project_file_path = base_path.join(Path::new(&file_name));

    if project_file_path.exists() {
        return Ok(());
    }

    //TODO: generalize for any chain
    let url =
        format!("https://sourcify.dev/server/v2/contract/1/{address}/?fields=sources,compilation");
    let project_json: Value = get(&url)?.json()?;

    let content = serde_json::to_string_pretty(&project_json)?;
    fs::create_dir_all(base_path)?;
    fs::write(project_file_path, content)?;

    Ok(())
}
