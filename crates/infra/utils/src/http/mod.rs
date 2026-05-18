use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};
use httpdate::fmt_http_date;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, IF_MODIFIED_SINCE, USER_AGENT};
use reqwest::StatusCode;

pub enum DownloadResult {
    Ok(Box<dyn Read>),
    NotModified,
    Error(reqwest::Error),
}

pub fn request_download_if_modified<P>(url: &str, path: P) -> DownloadResult
where
    P: AsRef<Path>,
{
    let client = Client::new();
    let mut request_builder = client.get(url);
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            request_builder = request_builder.header(IF_MODIFIED_SINCE, fmt_http_date(modified));
        }
    }

    let response = match request_builder.send() {
        Ok(response) => response,
        Err(error) => return DownloadResult::Error(error),
    };

    let status = response.status();
    if status.is_success() {
        DownloadResult::Ok(Box::new(response))
    } else if status.as_u16() == StatusCode::NOT_MODIFIED {
        DownloadResult::NotModified
    } else {
        DownloadResult::Error(
            response
                .error_for_status()
                .expect_err("expected the response to contain an error"),
        )
    }
}

/// Upload a crate to a registry that follows the crates.io publish protocol
/// (PUT a length-prefixed JSON metadata blob followed by a length-prefixed `.crate`).
/// `Authorization` header carries the registry token (the cargo OIDC short-lived
/// token in the publish workflow).
pub fn put_crate_to_registry(
    url: &str,
    metadata_json: &[u8],
    crate_bytes: &[u8],
    token: &str,
    user_agent: &str,
) -> Result<()> {
    let mut body = Vec::with_capacity(8 + metadata_json.len() + crate_bytes.len());
    body.extend_from_slice(&u32::try_from(metadata_json.len()).unwrap().to_le_bytes());
    body.extend_from_slice(metadata_json);
    body.extend_from_slice(&u32::try_from(crate_bytes.len()).unwrap().to_le_bytes());
    body.extend_from_slice(crate_bytes);

    let client = Client::builder()
        .user_agent(user_agent)
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .put(url)
        .header(AUTHORIZATION, token)
        .header(USER_AGENT, user_agent)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(body)
        .send()
        .with_context(|| format!("HTTP PUT failed: {url}"))?;

    let status = response.status();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        bail!("Registry rejected upload: HTTP {status}\n{body}");
    }
    println!("Registry accepted upload: HTTP {status}");
    Ok(())
}
