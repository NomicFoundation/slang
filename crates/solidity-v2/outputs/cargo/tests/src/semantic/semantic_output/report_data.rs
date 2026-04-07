use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Range;

use slang_solidity_v2_ir::ir::visitor::{accept_source_unit, Visitor};
use slang_solidity_v2_ir::ir::{Identifier, NodeId};
use slang_solidity_v2_parser::ParserError;
use slang_solidity_v2_semantic::binder::{Definition, Resolution, Typing};
use slang_solidity_v2_semantic::compilation::unit::CompilationUnit;
use slang_solidity_v2_semantic::types::{DataLocation, FunctionType, LiteralKind, Type, TypeId};

// Types

pub(crate) struct ReportData<'a> {
    pub(crate) compilation: &'a CompilationUnit,
    pub(crate) file_contents: &'a HashMap<String, String>,
    pub(crate) parse_errors: Vec<(String, ParserError)>,
    pub(crate) all_definitions: Vec<CollectedDefinition>,
    pub(crate) all_references: Vec<CollectedReference>,
    pub(crate) unbound_identifiers: Vec<CollectedIdentifier>,
    pub(crate) definitions_by_id: HashMap<NodeId, usize>,
}

#[derive(Clone)]
pub(crate) struct CollectedIdentifier {
    pub(crate) file_id: String,
    pub(crate) node_id: NodeId,
    pub(crate) range: Range<usize>,
    pub(crate) text: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

pub(crate) struct CollectedDefinition {
    pub(crate) report_id: usize,
    pub(crate) definition_node_id: NodeId,
    pub(crate) identifier: CollectedIdentifier,
}

pub(crate) struct CollectedReference {
    pub(crate) identifier: CollectedIdentifier,
    pub(crate) resolution: Resolution,
}

// Implementation

impl<'a> ReportData<'a> {
    pub(crate) fn prepare(
        compilation: &'a CompilationUnit,
        file_contents: &'a HashMap<String, String>,
        parse_errors: Vec<(String, ParserError)>,
    ) -> Self {
        let all_identifiers = collect_all_identifiers(compilation, file_contents);
        let (all_definitions, all_references, unbound_identifiers) =
            classify_identifiers(compilation, all_identifiers);
        let definitions_by_id = all_definitions
            .iter()
            .map(|definition| (definition.definition_node_id, definition.report_id))
            .collect::<HashMap<NodeId, usize>>();

        Self {
            compilation,
            file_contents,
            parse_errors,
            all_definitions,
            all_references,
            unbound_identifiers,
            definitions_by_id,
        }
    }

    pub(crate) fn all_resolved(&self) -> bool {
        self.parse_errors.is_empty()
            && self.unbound_identifiers.is_empty()
            && self
                .all_references
                .iter()
                .all(|reference| reference.resolution != Resolution::Unresolved)
    }
}

// Identifiers collection

struct IdentifierCollector<'a> {
    identifiers: Vec<CollectedIdentifier>,
    current_file: Option<(&'a String, &'a String)>,
}

impl Visitor for IdentifierCollector<'_> {
    fn visit_identifier(&mut self, node: &Identifier) {
        let (line, column) = self.byte_offset_to_line_column(node.range.start);
        self.identifiers.push(CollectedIdentifier {
            file_id: self.current_file_id(),
            node_id: node.id(),
            range: node.range.clone(),
            text: node.text.clone(),
            line,
            column,
        });
    }
}

impl IdentifierCollector<'_> {
    fn current_file_id(&self) -> String {
        let Some((file_id, _)) = self.current_file else {
            unreachable!("file is not set for collecting identifiers");
        };
        file_id.clone()
    }

    fn current_file_contents(&self) -> &String {
        let Some((_, contents)) = self.current_file else {
            unreachable!("file is not set for collecting identifiers");
        };
        contents
    }

    fn byte_offset_to_line_column(&self, byte_offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for (index, ch) in self.current_file_contents().char_indices() {
            if index >= byte_offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }
}

fn collect_all_identifiers(
    compilation: &CompilationUnit,
    file_contents: &HashMap<String, String>,
) -> Vec<CollectedIdentifier> {
    let mut collector = IdentifierCollector {
        identifiers: Vec::new(),
        current_file: None,
    };

    for file in &compilation.files() {
        collector.current_file = file_contents.get_key_value(file.id());
        accept_source_unit(file.ir_root(), &mut collector);
    }

    collector.identifiers
}

fn classify_identifiers(
    compilation: &CompilationUnit,
    all_identifiers: Vec<CollectedIdentifier>,
) -> (
    Vec<CollectedDefinition>,
    Vec<CollectedReference>,
    Vec<CollectedIdentifier>,
) {
    let mut all_definitions = Vec::new();
    let mut all_references = Vec::new();
    let mut unbound_identifiers = Vec::new();

    let binder = compilation.binder();

    // Walk all identifiers in file/source order and classify each one
    for identifier in all_identifiers {
        let mut bound = false;

        if let Some(definition) = binder.find_definition_by_identifier_node_id(identifier.node_id) {
            all_definitions.push(CollectedDefinition {
                report_id: all_definitions.len() + 1,
                definition_node_id: definition.node_id(),
                identifier: identifier.clone(),
            });
            bound = true;
        }

        if let Some(reference) = binder.find_reference_by_identifier_node_id(identifier.node_id) {
            all_references.push(CollectedReference {
                identifier: identifier.clone(),
                resolution: reference.resolution.clone(),
            });
            bound = true;
        }

        if !bound {
            unbound_identifiers.push(identifier);
        }
    }

    (all_definitions, all_references, unbound_identifiers)
}

// Display helpers

impl CollectedDefinition {
    pub(crate) fn display<'a>(
        &'a self,
        compilation: &'a CompilationUnit,
    ) -> CollectedDefinitionDisplay<'a> {
        CollectedDefinitionDisplay {
            definition: self,
            compilation,
        }
    }
}

pub(crate) struct CollectedDefinitionDisplay<'a> {
    definition: &'a CollectedDefinition,
    compilation: &'a CompilationUnit,
}

impl Display for CollectedDefinitionDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let identifier = &self.definition.identifier;
        write!(
            f,
            "Def: #{id} [\"{identifier}\" @ {file_id}:{line}:{column}] ({def_type})",
            id = self.definition.report_id,
            identifier = identifier.text,
            file_id = identifier.file_id,
            def_type = self.definition_type(),
            line = identifier.line,
            column = identifier.column,
        )
    }
}

impl CollectedDefinitionDisplay<'_> {
    fn definition_type(&self) -> String {
        if let Some(definition) = self
            .compilation
            .binder()
            .find_definition_by_id(self.definition.definition_node_id)
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
                Definition::YulParameter(_) => "yul parameter".to_string(),
                Definition::YulVariable(_) => "yul variable".to_string(),
            }
        } else {
            "(unknown)".to_string()
        }
    }

    fn definition_type_display(&self) -> String {
        let node_id = self.definition.definition_node_id;
        let typing = self.compilation.binder().node_typing(node_id);
        match typing {
            Typing::Unresolved => "unresolved".to_string(),
            Typing::Resolved(type_id) => self.type_display(type_id),
            Typing::This => "this".to_string(),
            Typing::Super => "super".to_string(),
            Typing::UserMetaType(meta_node_id) => {
                assert_eq!(meta_node_id, self.definition.definition_node_id);
                "meta-type".to_string()
            }
            _ => {
                unreachable!("unexpected typing {typing:?} of user definition");
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn type_display(&self, type_id: TypeId) -> String {
        match self.compilation.types().get_type_by_id(type_id) {
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
            Type::FixedSizeArray {
                element_type,
                size,
                location,
            } => format!(
                "{element_type}[{size}] {location}",
                element_type = self.type_display(*element_type),
                location = data_location_display(*location),
            ),
            Type::Function(FunctionType {
                parameter_types,
                return_type,
                ..
            }) => {
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
            Type::Literal(kind) => match kind {
                LiteralKind::Zero => "lit-zero".to_string(),
                LiteralKind::Rational => "rational".to_string(),
                LiteralKind::DecimalInteger => "lit-integer".to_string(),
                LiteralKind::HexInteger { bytes } => format!("lit-hex({bytes})"),
                LiteralKind::HexString { bytes } => format!("lit-hexstring({bytes})"),
                LiteralKind::String { bytes } => format!("lit-string({bytes})"),
                LiteralKind::Address => "lit-address".to_string(),
            },
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
        self.compilation
            .binder()
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .text
            .clone()
    }
}

impl CollectedReference {
    pub(crate) fn display<'a>(
        &'a self,
        definitions_by_id: &'a HashMap<NodeId, usize>,
    ) -> CollectedReferenceDisplay<'a> {
        CollectedReferenceDisplay {
            reference: self,
            definitions_by_id,
        }
    }
}

pub(crate) struct CollectedReferenceDisplay<'a> {
    reference: &'a CollectedReference,
    definitions_by_id: &'a HashMap<NodeId, usize>,
}

impl Display for CollectedReferenceDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let identifier = &self.reference.identifier;
        write!(
            f,
            "Ref: [\"{identifier}\" @ {file_id}:{line}:{column}] -> {definition}",
            identifier = identifier.text,
            file_id = identifier.file_id,
            definition = match &self.reference.resolution {
                Resolution::Unresolved => "unresolved".to_string(),
                Resolution::BuiltIn(_) => "built-in".to_string(),
                Resolution::Definition(definition_id) => {
                    format!(
                        "#{id}",
                        id = self.definitions_by_id.get(definition_id).unwrap()
                    )
                }
                Resolution::Ambiguous(definitions) => {
                    format!(
                        "refs: {ids:?}",
                        ids = definitions
                            .iter()
                            .map(|id| self.definitions_by_id.get(id).unwrap())
                            .collect::<Vec<_>>(),
                    )
                }
            },
            line = identifier.line,
            column = identifier.column,
        )
    }
}

impl CollectedIdentifier {
    pub(crate) fn display_unbound(&self) -> UnboundIdentifierDisplay<'_> {
        UnboundIdentifierDisplay { identifier: self }
    }
}

pub(crate) struct UnboundIdentifierDisplay<'a> {
    identifier: &'a CollectedIdentifier,
}

impl Display for UnboundIdentifierDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "???: [\"{identifier}\" @ {file_id}:{line}:{column}]",
            identifier = self.identifier.text,
            file_id = self.identifier.file_id,
            line = self.identifier.line,
            column = self.identifier.column,
        )
    }
}

fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}
