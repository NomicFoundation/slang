use std::fmt::{Error, Write};
use std::rc::Rc;

use codegen_language_definition::model::{
    filter_built_ins_for_version, BuiltIn, BuiltInContext, Language,
};
use semver::Version;

pub fn render_built_ins(language: &Rc<Language>) -> Result<String, Error> {
    let mut buffer = String::new();

    writeln!(buffer, "use metaslang_bindings::FileGraphBuilder;")?;
    writeln!(buffer, "use metaslang_bindings::ScopeGraphBuilder;")?;
    writeln!(buffer, "use metaslang_cst::kinds::KindTypes;")?;
    writeln!(buffer, "use semver::Version;")?;

    writeln!(buffer, "#[allow(clippy::too_many_lines)]")?;
    writeln!(
        buffer,
        "\npub fn define_built_ins<KT: KindTypes + 'static>(builder: &mut FileGraphBuilder<'_, KT>, scope: &mut ScopeGraphBuilder, version: &Version) {{",
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

fn tr(input: &str) -> String {
    match input {
        "int" => "int256".into(),
        "uint" => "uint256".into(),
        _ => input.replace('$', "%"),
    }
}

fn render_contexts_for_version(
    buffer: &mut String,
    built_in_contexts: &[BuiltInContext],
    version: &Version,
) -> Result<(), Error> {
    for context in built_in_contexts {
        writeln!(buffer, "    // {context}", context = context.name)?;
        writeln!(buffer, "    {{")?;
        if context.name == "$YulBuiltIns$" {
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
                        name = tr(&item.name),
                        return_type = item.return_type.as_ref().map_or(
                            String::from("None"),
                            |return_type| format!(
                                "Some(\"{return_type}\")",
                                return_type = tr(return_type)
                            )
                        ),
                    )?;
                }
                BuiltIn::BuiltInType { item } => {
                    if item.fields.is_empty() && item.functions.is_empty() {
                        writeln!(
                            buffer,
                            "      _ = scope.define_type(builder, \"{name}\");",
                            name = tr(&item.name),
                        )?;
                    } else {
                        writeln!(
                            buffer,
                            "      let mut type_scope = scope.define_type(builder, \"{name}\");",
                            name = tr(&item.name),
                        )?;
                        for field in &item.fields {
                            let parts = field.definition.split(' ').collect::<Vec<_>>();
                            writeln!(
                                buffer,
                                "      type_scope.define_field(builder, \"{name}\", \"{field_type}\");",
                                name = tr(parts.last().unwrap()),
                                field_type = tr(parts.first().unwrap()),
                            )?;
                        }
                        for function in &item.functions {
                            writeln!(
                                buffer,
                                "      type_scope.define_function(builder, \"{name}\", {return_type});",
                                name = tr(&function.name),
                                return_type = function
                                    .return_type
                                    .as_ref()
                                    .map_or(String::from("None"), |return_type| format!(
                                        "Some(\"{return_type}\")", return_type = tr(return_type),
                                    )),
                            )?;
                        }
                    }
                }
                BuiltIn::BuiltInVariable { item } => {
                    let parts = item.definition.split(' ').collect::<Vec<_>>();
                    writeln!(
                        buffer,
                        "      scope.define_field(builder, \"{name}\", \"{field_type}\");",
                        name = tr(parts.last().unwrap()),
                        field_type = tr(parts.first().unwrap()),
                    )?;
                }
            }
        }
        writeln!(buffer, "    }}")?;
    }
    Ok(())
}
