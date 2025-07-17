use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Write};
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity::backend::binder::{Definition, Resolution, Typing};
use slang_solidity::backend::passes::p4_resolve_references::Output;
use slang_solidity::backend::types::{DataLocation, Type, TypeId};
use slang_solidity::compilation::File;
use slang_solidity::cst::{Cursor, NodeId, NodeKind, TerminalKindExtensions};

const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

type Span<'a> = (&'a str, Range<usize>);
type BuilderType<'a> = ReportBuilder<'a, Span<'a>>;

pub(crate) fn binder_report(
    binder_data: &Output,
    compare_with_stack_graphs: bool,
) -> Result<(String, bool)> {
    let mut report = String::new();

    let (all_definitions, all_references, unbound_identifiers) =
        collect_all_definitions_and_references(binder_data);
    let definitions_by_id = all_definitions
        .iter()
        .map(|definition| (definition.definition_id, definition.report_id))
        .collect::<HashMap<NodeId, usize>>();

    report_all_definitions(&mut report, binder_data, &all_definitions)?;

    writeln!(report, "{SEPARATOR}")?;

    report_all_references(&mut report, &all_references, &definitions_by_id)?;

    writeln!(report, "{SEPARATOR}")?;

    report_unbound_identifiers(&mut report, &unbound_identifiers)?;

    for file in &binder_data.compilation_unit.files() {
        writeln!(report, "{SEPARATOR}")?;

        render_bindings_for_file(
            &mut report,
            file,
            &all_definitions,
            &all_references,
            &unbound_identifiers,
            &definitions_by_id,
        )?;
    }

    let all_resolved = unbound_identifiers.is_empty()
        && all_references
            .iter()
            .all(|reference| reference.resolution != Resolution::Unresolved);

    if compare_with_stack_graphs {
        writeln!(report, "{SEPARATOR}")?;

        report_binding_graph_differences(
            &mut report,
            binder_data,
            &all_definitions,
            &definitions_by_id,
        )?;
    }

    Ok((report, all_resolved))
}

fn report_all_definitions(
    report: &mut String,
    binder_data: &Output,
    all_definitions: &[CollectedDefinition],
) -> Result<()> {
    writeln!(
        report,
        "Definitions ({definitions_count}):",
        definitions_count = all_definitions.len(),
    )?;
    for definition in all_definitions {
        writeln!(
            report,
            "- {definition}",
            definition = definition.display(binder_data)
        )?;
    }
    Ok(())
}

fn report_all_references(
    report: &mut String,
    all_references: &[CollectedReference],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    writeln!(
        report,
        "References ({references_count}):",
        references_count = all_references.len()
    )?;
    for reference in all_references {
        writeln!(
            report,
            "- {reference}",
            reference = reference.display(definitions_by_id)
        )?;
    }
    Ok(())
}

fn report_unbound_identifiers(
    report: &mut String,
    unbound_identifiers: &[UnboundIdentifier],
) -> Result<()> {
    writeln!(
        report,
        "Unbound identifiers ({unbound_identifiers_count}):",
        unbound_identifiers_count = unbound_identifiers.len()
    )?;
    for unbound_identifier in unbound_identifiers {
        writeln!(
            report,
            "- {unbound_identifier}",
            unbound_identifier = unbound_identifier.display()
        )?;
    }
    Ok(())
}

fn render_bindings_for_file(
    report: &mut String,
    file: &File,
    all_definitions: &[CollectedDefinition],
    all_references: &[CollectedReference],
    unbound_identifiers: &[UnboundIdentifier],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    let contents = file.tree().unparse();
    let mut builder: BuilderType<'_> =
        Report::build(ReportKind::Custom("Bindings", Color::Unset), file.id(), 0)
            .with_config(Config::default().with_color(false));

    let new_label = |cursor: &Cursor, message: &str| -> Label<Span<'_>> {
        let range = {
            let range = cursor.text_range();
            let start = contents[..range.start.utf8].chars().count();
            let end = contents[..range.end.utf8].chars().count();
            start..end
        };
        Label::new((file.id(), range)).with_message(message)
    };

    for definition in all_definitions {
        if definition.file_id != file.id() {
            continue;
        }

        let message = format!(
            "name: {definition_id}",
            definition_id = definition.report_id
        );
        builder.add_label(new_label(&definition.cursor, &message));
    }

    for reference in all_references {
        if reference.file_id != file.id() {
            continue;
        }

        let message = match &reference.resolution {
            Resolution::Unresolved => "unresolved".to_string(),
            Resolution::BuiltIn(_) => "built-in".to_string(),
            Resolution::Definition(definition_id) => {
                format!(
                    "ref: {definition_id}",
                    definition_id = definitions_by_id.get(definition_id).unwrap()
                )
            }
            Resolution::Ambiguous(definitions) => {
                format!(
                    "refs: {ids:?}",
                    ids = definitions.iter().map(|id| definitions_by_id.get(id))
                )
            }
        };
        builder.add_label(new_label(&reference.cursor, &message));
    }

    for unbound_identifier in unbound_identifiers {
        if unbound_identifier.file_id != file.id() {
            continue;
        }

        builder.add_label(new_label(&unbound_identifier.cursor, "???"));
    }

    let mut buffer = Vec::<u8>::new();
    builder
        .finish()
        .write((file.id(), Source::from(contents)), &mut buffer)?;
    report.extend(String::from_utf8(buffer));

    Ok(())
}

fn report_binding_graph_differences(
    report: &mut String,
    binder_data: &Output,
    all_definitions: &[CollectedDefinition],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    let binding_graph = binder_data.compilation_unit.binding_graph();

    let mut has_diff = false;
    let mut binder_definitions = definitions_by_id
        .keys()
        .copied()
        .collect::<BTreeSet<NodeId>>();

    for graph_definition in binding_graph.all_definitions() {
        if graph_definition.get_file().is_built_ins() {
            continue;
        }
        let id = graph_definition.id();
        if binder_definitions.remove(&id) {
            continue;
        }
        has_diff = true;
        writeln!(report, "{graph_definition} not found in new binder output")?;
    }

    for definition_id in &binder_definitions {
        let report_id = definitions_by_id.get(definition_id).unwrap();
        let definition = &all_definitions[report_id - 1];
        writeln!(
            report,
            "{definition} not found in new stack graph output",
            definition = definition.display(binder_data),
        )?;
        has_diff = true;
    }

    writeln!(report)?;
    if has_diff {
        writeln!(
            report,
            "Definitions from binding graph and new binder DIFFER"
        )?;
    } else {
        writeln!(
            report,
            "No differences found in definitions from the binding graph"
        )?;
    }
    Ok(())
}

// Collection of definitions, references and unbound identifiers

fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}

struct CollectedDefinition {
    report_id: usize,
    cursor: Cursor,
    file_id: String,
    definition_id: NodeId,
}

impl CollectedDefinition {
    fn display<'a>(&'a self, binder_data: &'a Output) -> CollectedDefinitionDisplay<'a> {
        CollectedDefinitionDisplay {
            definition: self,
            binder_data,
        }
    }
}

struct CollectedDefinitionDisplay<'a> {
    definition: &'a CollectedDefinition,
    binder_data: &'a Output,
}

impl Display for CollectedDefinitionDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.definition.cursor.text_offset().line + 1;
        let column = self.definition.cursor.text_offset().column + 1;
        write!(
            f,
            "Def: #{id} [\"{identifier}\" @ {file_id}:{line}:{column}] ({def_type})",
            id = self.definition.report_id,
            identifier = self.definition.cursor.node().unparse(),
            file_id = self.definition.file_id,
            def_type = self.definition_type(),
        )
    }
}

impl CollectedDefinitionDisplay<'_> {
    fn definition_type(&self) -> String {
        if let Some(definition) = self
            .binder_data
            .binder
            .find_definition_by_id(self.definition.definition_id)
        {
            match definition {
                Definition::Constant(_) => {
                    format!("constant, type: {}", self.definition_type_display())
                }
                Definition::Contract(_) => "contract".to_string(),
                Definition::Enum(_) => "enum".to_string(),
                Definition::EnumMember(_) => {
                    format!("enum member of {}", self.definition_type_display())
                }
                Definition::Error(_) => "error".to_string(),
                Definition::Event(_) => "event".to_string(),
                Definition::Function(_) => {
                    format!("function, type: {}", self.definition_type_display())
                }
                Definition::Import(_) => "import".to_string(),
                Definition::ImportedSymbol(_) => "imported symbol".to_string(),
                Definition::Interface(_) => "interface".to_string(),
                Definition::Library(_) => "library".to_string(),
                Definition::Modifier(_) => "modifier".to_string(),
                Definition::Parameter(_) => {
                    format!("parameter, type: {}", self.definition_type_display())
                }
                Definition::StateVariable(_) => {
                    format!("state var, type: {}", self.definition_type_display())
                }
                Definition::Struct(_) => "struct".to_string(),
                Definition::StructMember(_) => {
                    format!("struct member, type: {}", self.definition_type_display())
                }
                Definition::TypeParameter(_) => "type param".to_string(),
                Definition::UserDefinedValueType(_) => {
                    format!("udvt, type: {}", self.definition_type_display())
                }
                Definition::Variable(_) => {
                    format!("variable, type: {}", self.definition_type_display())
                }
                Definition::YulFunction(_) => "yul function".to_string(),
                Definition::YulLabel(_) => "yul label".to_string(),
                Definition::YulParameter(_) => "yul parameter".to_string(),
                Definition::YulVariable(_) => "yul variable".to_string(),
            }
        } else {
            "(unknown)".to_string()
        }
    }

    fn definition_type_display(&self) -> String {
        let node_id = self.definition.definition_id;
        let typing = self.binder_data.binder.node_typing(node_id);
        match typing {
            Typing::Unresolved => "unresolved".to_string(),
            Typing::Resolved(type_id) => self.type_display(type_id),
            Typing::This => "this".to_string(),
            Typing::Super => "super".to_string(),
            Typing::Undetermined(type_ids) => {
                format!("unresolved {count} overloads", count = type_ids.len())
            }
            Typing::MetaType(node_id) => {
                assert_eq!(node_id, self.definition.definition_id);
                "meta-type".to_string()
            }
            Typing::BuiltIn(_built_in) => {
                // TODO: this should be unreachable because there should be no
                // definitions that type to a built-in
                "built-in".to_string()
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn type_display(&self, type_id: TypeId) -> String {
        match self.binder_data.types.get_type_by_id(type_id).unwrap() {
            Type::Address { payable } => {
                if *payable {
                    "address payable".to_string()
                } else {
                    "address".to_string()
                }
            }
            Type::Array {
                element_type,
                location,
            } => format!(
                "{element_type}[] {location}",
                element_type = self.type_display(*element_type),
                location = data_location_display(*location),
            ),
            Type::Boolean => "bool".to_string(),
            Type::ByteArray { width } => format!("bytes{width}"),
            Type::Bytes { location } => {
                format!(
                    "bytes {location}",
                    location = data_location_display(*location)
                )
            }
            Type::Contract { definition_id } => self.definition_name(*definition_id),
            Type::Enum { definition_id } => self.definition_name(*definition_id),
            Type::FixedPointNumber {
                signed,
                bits,
                precision_bits,
            } => {
                format!(
                    "{signed}fixed{bits}x{precision_bits}",
                    signed = if *signed { "" } else { "u" }
                )
            }
            Type::Function {
                parameter_types,
                return_type,
                ..
            } => {
                format!(
                    "function ({parameters}) returns {returns}",
                    parameters = parameter_types
                        .iter()
                        .map(|type_id| self.type_display(*type_id))
                        .collect::<Vec<_>>()
                        .join(", "),
                    returns = self.type_display(*return_type),
                )
            }
            Type::Integer { signed, bits } => {
                format!("{signed}int{bits}", signed = if *signed { "" } else { "u" })
            }
            Type::Interface { definition_id } => self.definition_name(*definition_id),
            Type::Mapping {
                key_type_id,
                value_type_id,
            } => {
                format!(
                    "{key} => {value}",
                    key = self.type_display(*key_type_id),
                    value = self.type_display(*value_type_id)
                )
            }
            Type::Rational => "rational".to_string(),
            Type::String { location } => {
                format!(
                    "string {location}",
                    location = data_location_display(*location)
                )
            }
            Type::Struct {
                definition_id,
                location,
            } => {
                format!(
                    "{name} {location}",
                    name = self.definition_name(*definition_id),
                    location = data_location_display(*location)
                )
            }
            Type::Tuple { types } => {
                format!(
                    "({types})",
                    types = types
                        .iter()
                        .map(|type_id| self.type_display(*type_id))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Type::UserDefinedValue { definition_id } => self.definition_name(*definition_id),
            Type::Void => "void".to_string(),
        }
    }

    fn definition_name(&self, definition_id: NodeId) -> String {
        self.binder_data
            .binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
    }
}

struct CollectedReference {
    cursor: Cursor,
    file_id: String,
    resolution: Resolution,
}

impl CollectedReference {
    fn display<'a>(
        &'a self,
        definitions_by_id: &'a HashMap<NodeId, usize>,
    ) -> CollectedReferenceDisplay<'a> {
        CollectedReferenceDisplay(self, definitions_by_id)
    }
}

struct CollectedReferenceDisplay<'a>(&'a CollectedReference, &'a HashMap<NodeId, usize>);

impl Display for CollectedReferenceDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.0.cursor.text_offset().line + 1;
        let column = self.0.cursor.text_offset().column + 1;
        write!(
            f,
            "Ref: [\"{identifier}\" @ {file_id}:{line}:{column}] -> {definition}",
            identifier = self.0.cursor.node().unparse(),
            file_id = self.0.file_id,
            definition = match &self.0.resolution {
                Resolution::Unresolved => "unresolved".to_string(),
                Resolution::BuiltIn(_) => "built-in".to_string(),
                Resolution::Definition(definition_id) => {
                    format!("#{id}", id = self.1.get(definition_id).unwrap())
                }
                Resolution::Ambiguous(definitions) => {
                    format!(
                        "refs: {ids:?}",
                        ids = definitions.iter().map(|id| self.1.get(id))
                    )
                }
            }
        )
    }
}

struct UnboundIdentifier {
    cursor: Cursor,
    file_id: String,
}

impl UnboundIdentifier {
    fn display(&self) -> UnboundIdentifierDisplay<'_> {
        UnboundIdentifierDisplay(self)
    }
}

struct UnboundIdentifierDisplay<'a>(&'a UnboundIdentifier);

impl Display for UnboundIdentifierDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.0.cursor.text_offset().line + 1;
        let column = self.0.cursor.text_offset().column + 1;
        write!(
            f,
            "???: [\"{identifier}\" @ {file_id}:{line}:{column}]",
            identifier = self.0.cursor.node().unparse(),
            file_id = self.0.file_id,
        )
    }
}

fn collect_all_definitions_and_references(
    binder_data: &Output,
) -> (
    Vec<CollectedDefinition>,
    Vec<CollectedReference>,
    Vec<UnboundIdentifier>,
) {
    let mut all_definitions = Vec::new();
    let mut all_references = Vec::new();
    let mut unbound_identifiers = Vec::new();

    for file in &binder_data.compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            let mut bound = false;
            let node_id = cursor.node().id();
            if let Some(definition) = binder_data
                .binder
                .find_definition_by_identifier_node_id(node_id)
            {
                all_definitions.push(CollectedDefinition {
                    report_id: all_definitions.len() + 1,
                    cursor: cursor.clone(),
                    file_id: file.id().to_string(),
                    definition_id: definition.node_id(),
                });
                bound = true;
            }
            if let Some(reference) = binder_data
                .binder
                .find_reference_by_identifier_node_id(node_id)
            {
                all_references.push(CollectedReference {
                    cursor: cursor.clone(),
                    file_id: file.id().to_string(),
                    resolution: reference.resolution.clone(),
                });
                bound = true;
            }

            if !bound {
                unbound_identifiers.push(UnboundIdentifier {
                    cursor: cursor.clone(),
                    file_id: file.id().to_string(),
                });
            }
        }
    }

    (all_definitions, all_references, unbound_identifiers)
}
