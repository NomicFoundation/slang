use std::collections::HashMap;
use std::fmt::Display;

use slang_solidity::backend::binder::{Definition, Resolution, Typing};
use slang_solidity::backend::passes::p5_resolve_references::Output;
use slang_solidity::backend::types::{DataLocation, FunctionType, LiteralKind, Type, TypeId};
use slang_solidity::cst::{Cursor, NodeId, NodeKind, TerminalKindExtensions};

// Types

pub(crate) struct ReportData<'a> {
    pub(crate) binder_data: &'a Output,
    pub(crate) all_definitions: Vec<CollectedDefinition>,
    pub(crate) all_references: Vec<CollectedReference>,
    pub(crate) unbound_identifiers: Vec<UnboundIdentifier>,
    pub(crate) definitions_by_id: HashMap<NodeId, usize>,
}

pub(crate) struct CollectedDefinition {
    pub(crate) report_id: usize,
    pub(crate) cursor: Cursor,
    pub(crate) file_id: String,
    pub(crate) definition_id: NodeId,
}

pub(crate) struct CollectedReference {
    pub(crate) cursor: Cursor,
    pub(crate) file_id: String,
    pub(crate) resolution: Resolution,
}

pub(crate) struct UnboundIdentifier {
    pub(crate) cursor: Cursor,
    pub(crate) file_id: String,
}

// Implementation

impl<'a> ReportData<'a> {
    pub(crate) fn prepare(binder_data: &'a Output) -> Self {
        let (all_definitions, all_references, unbound_identifiers) =
            collect_all_definitions_and_references(binder_data);
        let definitions_by_id = all_definitions
            .iter()
            .map(|definition| (definition.definition_id, definition.report_id))
            .collect::<HashMap<NodeId, usize>>();

        Self {
            binder_data,
            all_definitions,
            all_references,
            unbound_identifiers,
            definitions_by_id,
        }
    }

    pub(crate) fn all_resolved(&self) -> bool {
        self.unbound_identifiers.is_empty()
            && self
                .all_references
                .iter()
                .all(|reference| reference.resolution != Resolution::Unresolved)
    }

    pub(crate) fn has_parse_errors(&self) -> bool {
        self.binder_data
            .compilation_unit
            .files()
            .iter()
            .any(|file| !file.errors().is_empty())
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

impl CollectedDefinition {
    pub(crate) fn display<'a>(&'a self, binder_data: &'a Output) -> CollectedDefinitionDisplay<'a> {
        CollectedDefinitionDisplay {
            definition: self,
            binder_data,
        }
    }
}

pub(crate) struct CollectedDefinitionDisplay<'a> {
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
            Typing::UserMetaType(node_id) => {
                assert_eq!(node_id, self.definition.definition_id);
                "meta-type".to_string()
            }
            _ => {
                unreachable!("unexpected typing {typing:?} of user definition");
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn type_display(&self, type_id: TypeId) -> String {
        match self.binder_data.types.get_type_by_id(type_id) {
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
        self.binder_data
            .binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
    }
}

impl CollectedReference {
    pub(crate) fn display<'a>(
        &'a self,
        definitions_by_id: &'a HashMap<NodeId, usize>,
    ) -> CollectedReferenceDisplay<'a> {
        CollectedReferenceDisplay(self, definitions_by_id)
    }
}

pub(crate) struct CollectedReferenceDisplay<'a>(&'a CollectedReference, &'a HashMap<NodeId, usize>);

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
                        ids = definitions
                            .iter()
                            .map(|id| self.1.get(id).unwrap())
                            .collect::<Vec<_>>(),
                    )
                }
            }
        )
    }
}

impl UnboundIdentifier {
    pub(crate) fn display(&self) -> UnboundIdentifierDisplay<'_> {
        UnboundIdentifierDisplay(self)
    }
}

pub(crate) struct UnboundIdentifierDisplay<'a>(&'a UnboundIdentifier);

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

fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}
