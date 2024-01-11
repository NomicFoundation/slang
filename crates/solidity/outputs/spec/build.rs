use std::fmt::Write;
use std::path::Path;

use anyhow::Result;
use codegen_ebnf::{EbnfModel, PlainEbnfWriter};
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
    let ebnf_model = EbnfModel::build(language);

    let mut buffer = String::new();

    for (section_i, section) in language.sections.iter().enumerate() {
        let section_i = section_i + 1;
        let section_title = &section.title;

        writeln!(buffer, "(*")?;
        writeln!(buffer, " * {section_i}. {section_title}:")?;
        writeln!(buffer, " *)")?;
        writeln!(buffer)?;

        for (topic_i, topic) in section.topics.iter().enumerate() {
            let topic_i = topic_i + 1;
            let topic_title = &topic.title;

            writeln!(buffer, "(* {section_i}.{topic_i}. {topic_title}: *)")?;
            writeln!(buffer)?;

            if topic.items.is_empty() {
                writeln!(buffer, "(* No items *)")?;
                writeln!(buffer)?;
                continue;
            }

            for item in &topic.items {
                let mut writer = PlainEbnfWriter::default();
                ebnf_model.serialize(item.name(), &mut writer)?;
                writeln!(buffer, "{writer}")?;
            }
        }
    }

    codegen.write_file(output_dir.join("solidity.ebnf"), buffer)
}
