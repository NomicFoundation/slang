use std::fmt::Write;

use anyhow::Result;
use itertools::Itertools;

use crate::model::SpecModel;

pub fn generate_supported_versions(model: &SpecModel) -> Result<String> {
    let mut buffer = String::new();

    let versions = model
        .language
        .versions
        .iter()
        .map(|version| format!("`{version}`"))
        .collect_vec();

    writeln!(
        buffer,
        "This grammar compiles information from {versions_count} publicly released versions of {language_name}:", 
        versions_count = versions.len(),
        language_name = model.language.name,
    )?;

    writeln!(buffer)?;
    writeln!(buffer, "{list}", list = versions.join(" "))?;

    let breaking_versions = model
        .language
        .collect_all_breaking_versions()
        .iter()
        .map(|version| format!("`{version}`"))
        .collect_vec();

    writeln!(buffer)?;
    writeln!(
        buffer,
        "Among which, {count} versions have breaking changes:",
        count = breaking_versions.len(),
    )?;

    writeln!(buffer)?;
    writeln!(buffer, "{list}", list = breaking_versions.join(" "))?;

    Ok(buffer)
}
