use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::{
    BuiltIn, BuiltInContext, BuiltInField, BuiltInFunction, BuiltInType, Language,
};
use infra_utils::codegen::CodegenFileSystem;
use semver::Version;

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

fn filter_built_ins_for_version(built_ins: &[BuiltIn], version: &Version) -> Vec<BuiltIn> {
    built_ins
        .iter()
        .filter_map(|built_in| match built_in {
            BuiltIn::BuiltInFunction { item } => filter_built_in_function(item, version)
                .map(|item| BuiltIn::BuiltInFunction { item: item.into() }),
            BuiltIn::BuiltInType { item } => filter_built_in_type(item, version)
                .map(|item| BuiltIn::BuiltInType { item: item.into() }),
            BuiltIn::BuiltInVariable { item } => filter_built_in_field(item, version)
                .map(|item| BuiltIn::BuiltInVariable { item: item.into() }),
        })
        .collect()
}

fn filter_built_in_function(
    function: &BuiltInFunction,
    version: &Version,
) -> Option<BuiltInFunction> {
    if function
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        Some(function.clone())
    } else {
        None
    }
}

fn filter_built_in_field(field: &BuiltInField, version: &Version) -> Option<BuiltInField> {
    if field
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        Some(field.clone())
    } else {
        None
    }
}

fn filter_built_in_type(typ: &BuiltInType, version: &Version) -> Option<BuiltInType> {
    if typ
        .enabled
        .as_ref()
        .map_or(true, |enabled| enabled.contains(version))
    {
        let fields = typ
            .fields
            .iter()
            .filter_map(|field| filter_built_in_field(field, version))
            .collect();
        let functions = typ
            .functions
            .iter()
            .filter_map(|function| filter_built_in_function(function, version))
            .collect();

        Some(BuiltInType {
            name: typ.name.clone(),
            fields,
            functions,
            enabled: typ.enabled.clone(),
        })
    } else {
        None
    }
}
