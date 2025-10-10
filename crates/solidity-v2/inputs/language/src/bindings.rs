use std::fmt::{Error, Write};

use language_v2_definition::model::{
    filter_built_ins_for_version, BuiltIn, BuiltInContext, Language,
};
use semver::Version;

pub fn render_built_ins(language: &Language) -> Result<String, Error> {
    let mut buffer = String::new();

    writeln!(buffer, "use metaslang_bindings::FileGraphBuilder;")?;
    writeln!(buffer, "use metaslang_bindings::ScopeGraphBuilder;")?;
    writeln!(buffer, "use metaslang_cst::kinds::KindTypes;")?;
    writeln!(buffer, "use semver::Version;")?;

    writeln!(buffer, "#[allow(clippy::too_many_lines)]")?;
    writeln!(
        buffer,
        "pub fn define_built_ins<KT: KindTypes + 'static>(builder: &mut FileGraphBuilder<'_, KT>, scope: &mut ScopeGraphBuilder, version: &Version) {{",
    )?;
    let versions = language.collect_built_ins_versions();
    let versions = versions.iter().collect::<Vec<_>>();
    for vs in versions.windows(2) {
        let version = vs[0];
        let next_version = vs[1];
        writeln!(
            buffer,
            "if *version < Version::new({major}, {minor}, {patch}) {{",
            major = next_version.major,
            minor = next_version.minor,
            patch = next_version.patch
        )?;
        writeln!(buffer, "    // {version}")?;

        render_contexts_for_version(&mut buffer, &language.built_ins, version)?;

        write!(buffer, "  }} else ")?;
    }
    writeln!(buffer, "{{")?;
    let last_version = versions.last().unwrap();
    writeln!(buffer, "  // {last_version}")?;
    render_contexts_for_version(&mut buffer, &language.built_ins, last_version)?;
    writeln!(buffer, "  }}")?;
    writeln!(buffer, "}}")?;

    Ok(buffer)
}

fn canonicalize_type(input: &str) -> String {
    match input {
        "int" => "int256".into(),
        "uint" => "uint256".into(),
        _ => input.replace(" memory", ""),
    }
}

fn split_type_and_name(input: &str) -> (&str, &str) {
    let Some(last_space_index) = input.rfind(' ') else {
        unreachable!("Invalid field/variable definition");
    };
    (&input[0..last_space_index], &input[last_space_index + 1..])
}

fn optional_type(type_name: Option<&String>) -> String {
    type_name
        .as_ref()
        .map_or(String::from("None"), |return_type| {
            format!(
                "Some(\"{return_type}\")",
                return_type = canonicalize_type(return_type)
            )
        })
}

fn render_contexts_for_version(
    buffer: &mut String,
    built_in_contexts: &[BuiltInContext],
    version: &Version,
) -> Result<(), Error> {
    for context in built_in_contexts {
        writeln!(buffer, "    // {context}", context = context.name)?;
        writeln!(buffer, "    {{")?;
        // __SLANG_YUL_BUILT_INS_CONTEXT_NAME__ keep in sync with language definition
        if context.name == "YulBuiltIns" {
            // __SLANG_SOLIDITY_YUL_BUILT_INS_GUARD__ keep in sync with binding rules
            writeln!(
                buffer,
                "      let mut scope = scope.new_context(builder, \"@yul\");"
            )?;
        }
        for item in filter_built_ins_for_version(&context.definitions, version) {
            match item {
                BuiltIn::BuiltInFunction { item } => {
                    writeln!(
                        buffer,
                        "      scope.define_function(builder, \"{name}\", {return_type});",
                        name = item.name,
                        return_type = optional_type(item.return_type.as_ref()),
                    )?;
                }
                BuiltIn::BuiltInType { item } => {
                    if item.fields.is_empty() && item.functions.is_empty() {
                        writeln!(
                            buffer,
                            "      _ = scope.define_type(builder, \"{name}\");",
                            name = item.name,
                        )?;
                    } else {
                        writeln!(
                            buffer,
                            "      let mut type_scope = scope.define_type(builder, \"{name}\");",
                            name = item.name,
                        )?;
                        for field in &item.fields {
                            let (field_type, name) = split_type_and_name(&field.definition);
                            writeln!(
                                buffer,
                                "      type_scope.define_field(builder, \"{name}\", \"{field_type}\");",
                                field_type = canonicalize_type(field_type),
                            )?;
                        }
                        for function in &item.functions {
                            writeln!(
                                buffer,
                                "      type_scope.define_function(builder, \"{name}\", {return_type});",
                                name = function.name,
                                return_type = optional_type(function.return_type.as_ref()),
                            )?;
                        }
                    }
                }
                BuiltIn::BuiltInVariable { item } => {
                    let (field_type, name) = split_type_and_name(&item.definition);
                    writeln!(
                        buffer,
                        "      scope.define_field(builder, \"{name}\", \"{field_type}\");",
                        field_type = canonicalize_type(field_type),
                    )?;
                }
            }
        }
        writeln!(buffer, "    }}")?;
    }
    Ok(())
}
