use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use itertools::Itertools;
use markdown::{Block, Span};
use semver::Version;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct GithubReleaseController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl GithubReleaseController {
    pub fn execute(&self) -> Result<()> {
        let current_version = CargoWorkspace::local_version()?;
        println!("Current version: {current_version}");

        let previous_version = GitHub::latest_release_version()?;
        println!("Latest published version: {previous_version}");

        if current_version == previous_version {
            println!("Skipping release, since the workspace version is already published.");
            return Ok(());
        }

        let notes = extract_latest_changelogs(&current_version, &previous_version)?;
        let tag_name = format!("v{current_version}");

        println!("Creating release '{tag_name}' with contents:");
        println!();
        println!("{}", notes.lines().map(|l| format!("  │ {l}")).join("\n"));
        println!();

        if self.dry_run.get() {
            return Ok(());
        }

        GitHub::create_new_release(tag_name, notes)
    }
}

fn extract_latest_changelogs(
    current_version: &Version,
    previous_version: &Version,
) -> Result<String> {
    let changelog_contents = Path::repo_path("CHANGELOG.md").read_to_string()?;

    let mut all_blocks = markdown::tokenize(&changelog_contents).into_iter();

    // Assert that first block contains title '# changelog'
    assert_eq!(
        all_blocks.next().unwrap(),
        Block::Header(vec![Span::Text("changelog".to_string())], 1),
    );

    // H2 for current_version: '## 1.2.3'
    assert_eq!(
        all_blocks.next().unwrap(),
        Block::Header(vec![Span::Text(format!("{current_version}"))], 2),
    );

    let latest_version_blocks = all_blocks
        .take_while(|block| match block {
            // H2 for previous_version: '## 1.2.3'
            Block::Header(contents, level) if *level == 2 => {
                assert_eq!(contents, &vec![Span::Text(format!("{previous_version}"))]);
                false
            }
            // H3 for change kinds: breaking changes, features, or fixes.
            Block::Header(_, level) if *level == 3 => true,
            // Individual changelog entries.
            Block::UnorderedList(_) => true,
            _ => panic!("Unexpected block: {block:#?}"),
        })
        .collect::<Vec<_>>();

    Ok(markdown::generate_markdown(latest_version_blocks))
}
