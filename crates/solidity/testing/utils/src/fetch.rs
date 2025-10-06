use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use anyhow::{bail, Result};
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

const MAX_RETRIES: u64 = 6;

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

    // Try with exponential backoff
    let mut retries = 0;
    let project = loop {
        retries += 1;

        match try_fetch_project(&url) {
            Ok(project) => break project,
            Err(err) => {
                println!("Error fetching {url} (attempt {retries}/{MAX_RETRIES}): {err}");
                if retries <= MAX_RETRIES {
                    thread::sleep(Duration::from_secs(2 ^ retries));
                } else {
                    bail!("Giving up after {retries} attempts.");
                }
            }
        }
    };

    // write project to disk
    fs::create_dir_all(base_path)?;
    fs::write(project_file_path, project)?;

    Ok(())
}

fn try_fetch_project(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)?;

    if response.status() != reqwest::StatusCode::OK {
        bail!("Status code is {status}", status = response.status());
    }

    let Some(content_type) = response.headers().get(CONTENT_TYPE) else {
        bail!("No content-type header")
    };

    let content_type_str = content_type.to_str().unwrap_or("");
    if !content_type_str
        .trim()
        .to_ascii_lowercase()
        .starts_with("application/json")
    {
        bail!("content-type is not json: {content_type_str:?}");
    }

    if response.content_length().unwrap_or_default() == 0 {
        bail!("body is empty");
    }

    let project_json: Value = response.json()?;

    Ok(serde_json::to_string_pretty(&project_json)?)
}
