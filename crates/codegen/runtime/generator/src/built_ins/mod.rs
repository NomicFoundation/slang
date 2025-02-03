use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::{filter_built_ins_for_version, BuiltInContext, Language};
use infra_utils::codegen::CodegenFileSystem;

pub fn render_built_ins(
    file_system: &mut CodegenFileSystem,
    language: &Rc<Language>,
    output_dir: &Path,
    render_fn: impl Fn(&[BuiltInContext]) -> String,
) -> Result<()> {
    let versions = language.collect_built_ins_versions();
    let file_extension = language.file_extension.clone().unwrap_or_default();
    for version in &versions {
        let built_in_contexts = language
            .built_ins
            .iter()
            .map(|context| {
                let definitions = filter_built_ins_for_version(&context.definitions, version);
                BuiltInContext {
                    definitions,
                    ..context.clone()
                }
            })
            .collect::<Vec<_>>();
        let contents = render_fn(&built_in_contexts);

        let output_path = output_dir.join(format!("{version}{file_extension}"));
        file_system.write_file(output_path, contents)?;
    }
    Ok(())
}
