use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};
use httpdate::fmt_http_date;
use reqwest::blocking::Client;
use reqwest::header::IF_MODIFIED_SINCE;
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

/// One line of the crates.io sparse-index JSONL response. We only consume
/// `vers`, but the full schema is at
/// <https://doc.rust-lang.org/cargo/reference/registry-index.html#json-schema>.
#[derive(serde::Deserialize)]
struct SparseIndexEntry {
    vers: String,
}

/// Look up the published versions of a crate in the crates.io sparse index.
/// Returns the `vers` field from each line of the JSONL response (one version
/// per line, including yanked ones).
///
/// Uses `index.crates.io` (the sparse index), not the search API, so this is
/// safe to poll: it's CDN-cached, designed for high-frequency reads, and does
/// not count toward search rate limits. Path scheme:
/// <https://doc.rust-lang.org/cargo/reference/registry-index.html#index-files>
pub fn fetch_index_versions(crate_name: &str) -> Result<Vec<String>> {
    let url = sparse_index_url(crate_name);
    let client = Client::builder()
        .user_agent("slang-infra-publish")
        .build()
        .context("Failed to build HTTP client")?;
    let response = client
        .get(&url)
        .send()
        .with_context(|| format!("HTTP GET failed: {url}"))?;
    let status = response.status();
    if status == StatusCode::NOT_FOUND {
        // Crate not yet on the index (e.g. first-time publish or propagation lag).
        return Ok(vec![]);
    }
    if !status.is_success() {
        bail!("crates.io sparse index returned HTTP {status} for {url}");
    }
    let body = response.text()?;
    let mut versions = vec![];
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let entry: SparseIndexEntry = serde_json::from_str(line)
            .with_context(|| format!("Failed to parse sparse-index line: {line}"))?;
        versions.push(entry.vers);
    }
    Ok(versions)
}

/// Build the sparse-index URL for a crate per the cargo registry layout.
/// Exposed only so we can unit-test the path-prefix scheme; callers should
/// use `fetch_index_versions`.
fn sparse_index_url(crate_name: &str) -> String {
    let lower = crate_name.to_lowercase();
    let prefix = match lower.len() {
        0 => panic!("empty crate name"),
        1 => format!("1/{lower}"),
        2 => format!("2/{lower}"),
        3 => format!("3/{first}/{lower}", first = &lower[..1]),
        _ => format!("{a}/{b}/{lower}", a = &lower[..2], b = &lower[2..4],),
    };
    format!("https://index.crates.io/{prefix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_index_url_path_scheme() {
        // Length-based prefixes per the cargo registry layout.
        assert_eq!(sparse_index_url("a"), "https://index.crates.io/1/a");
        assert_eq!(sparse_index_url("ab"), "https://index.crates.io/2/ab");
        assert_eq!(sparse_index_url("abc"), "https://index.crates.io/3/a/abc");
        assert_eq!(
            sparse_index_url("metaslang_cst"),
            "https://index.crates.io/me/ta/metaslang_cst"
        );
        assert_eq!(
            sparse_index_url("slang_solidity"),
            "https://index.crates.io/sl/an/slang_solidity"
        );
    }
}
