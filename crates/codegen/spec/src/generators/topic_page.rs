use std::fmt::{Display, Write};

use anyhow::Result;
use codegen_ebnf::EbnfWriter;
use language_definition::model::Identifier;

use crate::model::SpecModel;

pub fn generate_topic_page(
    model: &SpecModel,
    section_index: usize,
    topic_index: usize,
) -> Result<String> {
    let section = &model.sections[section_index];
    let topic = &section.topics[topic_index];
    assert!(!topic.items.is_empty());

    let mut buffer = String::new();

    writeln!(buffer, "# {topic_title}", topic_title = topic.title)?;

    for item in &topic.items {
        writeln!(buffer)?;
        writeln!(buffer, "```{{ .ebnf #{item} }}")?;
        writeln!(buffer)?;
        writeln!(buffer, "```")?;

        // Markdown code blocks don't support links between identifiers.
        // So in order to do that, we generate the links into a hidden element,
        // then after page load, we inject this snippet's contents into the code block.

        writeln!(buffer)?;
        write!(
            buffer,
            "<pre ebnf-snippet=\"{item}\" style=\"display: none;\">",
        )?;

        model.ebnf.serialize(
            item,
            &mut HtmlWriter {
                buffer: &mut buffer,
                model,
                section_index,
                topic_index,
            },
        )?;

        writeln!(buffer, "</pre>")?;
    }

    Ok(buffer)
}

struct HtmlWriter<'m> {
    buffer: &'m mut String,
    model: &'m SpecModel,

    section_index: usize,
    topic_index: usize,
}

impl EbnfWriter for HtmlWriter<'_> {
    fn line_break(&mut self) -> std::fmt::Result {
        write!(self.buffer, "<br />")
    }

    fn write_comment(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "<span class=\"cm\">{value}</span>")
    }

    fn write_punctuation(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "<span class=\"o\">{value}</span>")
    }

    fn write_string_literal(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "<span class=\"s2\">{value}</span>")
    }

    fn write_identifier(&mut self, value: impl Display, name: &Identifier) -> std::fmt::Result {
        let (target_section_i, target_topic_i) = self.model.ebnf.locate(name);

        let section_slug = &self.model.sections[target_section_i].slug;
        let topic_slug = &self.model.sections[target_section_i].topics[target_topic_i].slug;

        let link = if (target_section_i, target_topic_i) == (self.section_index, self.topic_index) {
            format!("#{name}")
        } else if target_section_i == self.section_index {
            format!("../{topic_slug}#{name}")
        } else {
            format!("../../{section_slug}/{topic_slug}#{name}")
        };

        write!(self.buffer, "<a href=\"{link}\">")?;
        write!(self.buffer, "<span class=\"k\">{value}</span>")?;
        write!(self.buffer, "</a>")?;

        Ok(())
    }
}
