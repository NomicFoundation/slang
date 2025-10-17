use std::fs;
use std::io::Read;
use std::path::Path;

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
