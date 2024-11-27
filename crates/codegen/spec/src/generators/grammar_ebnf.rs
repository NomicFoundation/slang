use std::fmt::Write;

use anyhow::Result;
use codegen_ebnf::PlainWriter;

use crate::model::SpecModel;

pub fn generate_grammar_ebnf(model: &SpecModel<'_>) -> Result<String> {
    let mut buffer = String::new();

    let mut plain_writer = PlainWriter::default();

    for section in &model.sections {
        writeln!(buffer, "(*")?;
        writeln!(buffer, " * {title}:", title = section.title)?;
        writeln!(buffer, " *)")?;
        writeln!(buffer)?;

        for topic in &section.topics {
            writeln!(buffer, "(* {title}: *)", title = topic.title)?;
            writeln!(buffer)?;

            if topic.items.is_empty() {
                writeln!(buffer, "(* No items *)")?;
                writeln!(buffer)?;
                continue;
            }

            for item in &topic.items {
                model.ebnf.serialize(item, &mut plain_writer)?;
                writeln!(buffer, "{ebnf}", ebnf = plain_writer.flush())?;
                writeln!(buffer)?;
            }
        }
    }

    Ok(buffer)
}
