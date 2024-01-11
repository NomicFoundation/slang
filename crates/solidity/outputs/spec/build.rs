use std::path::Path;

use anyhow::Result;
use codegen_ebnf::{EbnfModel, EbnfWriter};
use codegen_language_definition::model::Language;
use codegen_spec::SpecGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{Codegen, CodegenWriteOnly};
use solidity_language::{SolidityDefinition, SolidityLanguageExtensions};

fn main() -> Result<()> {
    let mut codegen = Codegen::write_only()?;
    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    // TODO: combine both ebnf and spec codegen into 'codegen_spec' crate:
    let language_v0 = codegen_schema::types::LanguageDefinition::load_solidity()?;
    language_v0.generate_spec(&mut codegen, &output_dir)?;

    let language_v2 = SolidityDefinition::create();
    generate_language_ebnf(&mut codegen, &language_v2, &output_dir)?;

    Ok(())
}

fn generate_language_ebnf(
    codegen: &mut CodegenWriteOnly,
    language: &Language,
    output_dir: &Path,
) -> Result<()> {
    let mut writer = SimpleEbnfWriter {
        buffer: String::new(),
    };
    let ebnf_model = EbnfModel::build(language);

    for (section_index, section) in language.sections.iter().enumerate() {
        writer.buffer.push_str(&format!(
            "(*\n * {section_index}. {section_title}:\n *)\n\n",
            section_index = section_index + 1,
            section_title = section.title,
        ));

        for (topic_index, topic) in section.topics.iter().enumerate() {
            writer.buffer.push_str(&format!(
                "(* {section_index}.{topic_index}. {topic_title}: *)\n\n",
                section_index = section_index + 1,
                topic_index = topic_index + 1,
                topic_title = topic.title,
            ));

            for item in &topic.items {
                ebnf_model.serialize(item.name(), &mut writer);
                writer.buffer.push('\n');
            }
        }
    }

    codegen.write_file(output_dir.join("solidity.ebnf"), writer.buffer)
}

struct SimpleEbnfWriter {
    buffer: String,
}

impl EbnfWriter for SimpleEbnfWriter {
    fn start_line(&mut self) {
        // No-op
    }

    fn end_line(&mut self) {
        self.buffer.push('\n');
    }

    fn write_comment(&mut self, value: String) {
        self.buffer.push_str(&value);
    }

    fn write_identifier(&mut self, value: &str, _name: &str) {
        self.buffer.push_str(value);
    }

    fn write_punctuation(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    fn write_string_literal(&mut self, value: String) {
        self.buffer.push_str(&value);
    }
}
