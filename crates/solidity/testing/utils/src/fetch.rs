use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use anyhow::{anyhow, bail, Ok, Result};
use reqwest::blocking::get;
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

    let mut body = get(&url);

    // Try with exponential backoff
    let mut tries = 0;
    while (body.is_err() || body.as_ref().unwrap().status() != reqwest::StatusCode::OK)
        && tries < MAX_RETRIES
    {
        tries += 1;
        thread::sleep(Duration::from_secs(2 ^ tries));
        body = get(&url);
    }

    let body = body.map_err(|e| anyhow!("Error fetching {url}: {e}"))?;

    if tries == MAX_RETRIES {
        bail!(
            "Error fetching {url}: status code is {status}",
            status = body.status(),
        );
    }

    let Some(content_type) = body.headers().get(CONTENT_TYPE) else {
        bail!("Error fetching {url}: no content-type header")
    };

    let content_type_str = content_type.to_str().unwrap_or("");
    if !content_type_str
        .trim()
        .to_ascii_lowercase()
        .starts_with("application/json")
    {
        return Err(anyhow!(
            "Error fetching {url}: content-type is {content_type_str:?}"
        ));
    }

    if body.content_length().unwrap_or_default() == 0 {
        return Err(anyhow!("Error fetching {url}: body is empty"));
    }

    let project_json: Value = body.json()?;

    let content = serde_json::to_string_pretty(&project_json)?;
    fs::create_dir_all(base_path)?;
    fs::write(project_file_path, content)?;

    Ok(())
}
