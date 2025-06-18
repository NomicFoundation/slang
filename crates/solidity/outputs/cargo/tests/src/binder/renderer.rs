use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity::backend::passes::p3_resolve_untyped::Output;
use slang_solidity::cst::{Cursor, NodeId, NodeKind, NonterminalKind, TerminalKindExtensions};

const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

type Span<'a> = (&'a str, Range<usize>);
type BuilderType<'a> = ReportBuilder<'a, Span<'a>>;

pub(crate) fn binder_report(binder_data: &Output) -> Result<String> {
    let mut report = String::new();

    let (all_definitions, all_references, unbound_identifiers) =
        collect_all_definitions_and_references(binder_data);
    let definitions_by_id = all_definitions
        .iter()
        .map(|definition| (definition.definition_id, definition.report_id))
        .collect::<HashMap<NodeId, usize>>();

    writeln!(
        report,
        "Definitions ({definitions_count}):",
        definitions_count = all_definitions.len(),
    )?;
    for definition in &all_definitions {
        writeln!(report, "- {definition}", definition = definition.display())?;
    }

    writeln!(report, "{SEPARATOR}")?;

    writeln!(
        report,
        "References ({references_count}):",
        references_count = all_references.len()
    )?;
    for reference in &all_references {
        writeln!(
            report,
            "- {reference}",
            reference = reference.display(&definitions_by_id)
        )?;
    }

    writeln!(report, "{SEPARATOR}")?;

    writeln!(
        report,
        "Unbound identifiers ({unbound_identifiers_count}):",
        unbound_identifiers_count = unbound_identifiers.len()
    )?;
    for unbound_identifier in &unbound_identifiers {
        writeln!(
            report,
            "- {unbound_identifier}",
            unbound_identifier = unbound_identifier.display()
        )?;
    }

    for file in &binder_data.compilation_unit.files() {
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

        for definition in &all_definitions {
            if definition.file_id != file.id() {
                continue;
            }

            let message = format!(
                "name: {definition_id}",
                definition_id = definition.report_id
            );
            builder.add_label(new_label(&definition.cursor, &message));
        }

        for reference in &all_references {
            if reference.file_id != file.id() {
                continue;
            }

            let message = if let Some(definition_id) = reference
                .definition_id
                .and_then(|node_id| definitions_by_id.get(&node_id))
            {
                format!("ref: {definition_id}")
            } else {
                "unresolved".to_string()
            };
            builder.add_label(new_label(&reference.cursor, &message));
        }

        for unbound_identifier in &unbound_identifiers {
            if unbound_identifier.file_id != file.id() {
                continue;
            }

            builder.add_label(new_label(&unbound_identifier.cursor, "???"));
        }

        writeln!(report, "{SEPARATOR}")?;
        let mut buffer = Vec::<u8>::new();
        builder
            .finish()
            .write((file.id(), Source::from(contents)), &mut buffer)?;
        report.extend(String::from_utf8(buffer));
    }

    Ok(report)
}

struct CollectedDefinition {
    report_id: usize,
    cursor: Cursor,
    file_id: String,
    definition_id: NodeId,
}

impl CollectedDefinition {
    fn display(&self) -> CollectedDefinitionDisplay<'_> {
        CollectedDefinitionDisplay(self)
    }
}

struct CollectedDefinitionDisplay<'a>(&'a CollectedDefinition);

impl Display for CollectedDefinitionDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.0.cursor.text_offset().line + 1;
        let column = self.0.cursor.text_offset().column + 1;
        write!(
            f,
            "Def: #{id} [\"{identifier}\" @ {file_id}:{line}:{column}]",
            id = self.0.report_id,
            identifier = self.0.cursor.node().unparse(),
            file_id = self.0.file_id,
        )
    }
}

struct CollectedReference {
    cursor: Cursor,
    file_id: String,
    definition_id: Option<NodeId>,
}

impl CollectedReference {
    fn display(&self, definitions_by_id: &HashMap<NodeId, usize>) -> CollectedReferenceDisplay<'_> {
        CollectedReferenceDisplay(
            self,
            self.definition_id
                .and_then(|node_id| definitions_by_id.get(&node_id).copied()),
        )
    }
}

struct CollectedReferenceDisplay<'a>(&'a CollectedReference, Option<usize>);

impl Display for CollectedReferenceDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.0.cursor.text_offset().line + 1;
        let column = self.0.cursor.text_offset().column + 1;
        write!(
            f,
            "Ref: [\"{identifier}\" @ {file_id}:{line}:{column}] -> {definition}",
            identifier = self.0.cursor.node().unparse(),
            file_id = self.0.file_id,
            definition = self
                .1
                .map_or("unresolved".to_string(), |id| format!("#{id}")),
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
            if matches!(
                cursor.ancestors().next(),
                Some(ancestor)
                    if ancestor.kind == NonterminalKind::ExperimentalFeature
            ) {
                // ignore identifiers in `pragma experimental` directives,
                // as they are unbound feature names:
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
                    definition_id: reference.definition_id,
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
