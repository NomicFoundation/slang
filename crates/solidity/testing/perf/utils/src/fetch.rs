use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

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

    //NOTE: This only fetches projects from mainnet (1). In the future we might want to make it generic.
    let url =
        format!("https://sourcify.dev/server/v2/contract/1/{address}/?fields=sources,compilation");

    let mut body = get(&url);

    // Try with exponential backoff
    let mut tries = 0;
    while body.is_err() && tries < 6 {
        tries += 1;
        thread::sleep(Duration::from_secs(2 ^ tries));
        body = get(&url);
    }

    let project_json: Value = body?.json()?;

    let content = serde_json::to_string_pretty(&project_json)?;
    fs::create_dir_all(base_path)?;
    fs::write(project_file_path, content)?;

    Ok(())
}
