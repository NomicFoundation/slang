mod git;

use std::path::PathBuf;

use anyhow::Result;
use infra_utils::github::GitHub;
use url::Url;

use crate::datasets::git::GitDataset;

pub trait Dataset {
    fn get_title(&self) -> &'static str;
    fn prepare(&self) -> Result<Vec<PathBuf>>;
}

pub fn get_all_datasets() -> Result<Vec<impl Dataset>> {
    let mut result = vec![
        GitDataset::new(
            "sanctuary-arbitrum",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-arbitrum")?,
        ),
        GitDataset::new(
            "sanctuary-avalanche",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-avalanche")?,
        ),
        GitDataset::new(
            "sanctuary-fantom",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-fantom")?,
        ),
        GitDataset::new(
            "sanctuary-tron",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-tron")?,
        ),
        GitDataset::new(
            "sanctuary-optimism",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-optimism")?,
        ),
        GitDataset::new(
            "sanctuary-celo",
            Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-celo")?,
        ),
    ];

    // Large repositories (millions of files). Skip running locally for now.
    if GitHub::is_running_in_ci() {
        result.extend(
            [
                GitDataset::new(
                    "sanctuary-ethereum",
                    Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-ethereum")?,
                ),
                GitDataset::new(
                    "sanctuary-bsc",
                    Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-bsc")?,
                ),
                GitDataset::new(
                    "sanctuary-polygon",
                    Url::parse("https://github.com/tintinweb/smart-contract-sanctuary-polygon")?,
                ),
            ],
        );
    }

    return Ok(result);
}
