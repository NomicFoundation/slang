use std::fmt::Write;

use anyhow::Result;

use crate::model::SpecModel;

pub fn generate_top_navigation(model: &SpecModel) -> Result<String> {
    let mut buffer = String::new();

    writeln!(buffer, "-   [Supported Versions](./supported-versions.md)")?;

    for section in &model.sections {
        writeln!(
            buffer,
            "-   [{section_title}](./{section_slug}/)",
            section_title = section.title,
            section_slug = section.slug,
        )?;
    }

    Ok(buffer)
}

pub fn generate_section_navigation(model: &SpecModel, section_index: usize) -> Result<String> {
    let mut buffer = String::new();

    for topic in &model.sections[section_index].topics {
        writeln!(
            buffer,
            "-   [{topic_title}](./{topic_slug}.md)",
            topic_title = topic.title,
            topic_slug = topic.slug,
        )?;
    }

    Ok(buffer)
}
