use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use semver::Version;

use self::ast::{create_contract_definition, create_source_unit, ContractDefinition, Definition};
use super::abi::ContractAbi;
use crate::backend::binder::Binder;
pub use crate::backend::ir::{ast, ir2_flat_contracts as output_ir};
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::backend::{binder, passes};
use crate::compilation::File;
use crate::cst::{Cursor, NodeId, NonterminalNode, TextIndex};
use crate::parser::ParseError;

// TODO(v2): Unify with `File` as follows
// (see https://github.com/NomicFoundation/slang/pull/1477#discussion_r2633584612):
// 1. Rename `File` to a `FileBuilder`, which will contain the CST root.
// 2. Rename this `SemanticFile` to `File`, which will contain both the CST root and the AST root.
// 3. When user calls `CompilationBuilder::build()`, we can convert 1. to 2. while
//    running the backend passes.
#[derive(Clone)]
pub struct SemanticFile {
    file: Rc<File>,
    ir_root: output_ir::SourceUnit,
}

impl SemanticFile {
    pub fn id(&self) -> &str {
        self.file.id()
    }

    pub fn file(&self) -> &Rc<File> {
        &self.file
    }

    pub(crate) fn ir_root(&self) -> &output_ir::SourceUnit {
        &self.ir_root
    }

    /// Returns the syntax tree of this file.
    pub fn tree(&self) -> &Rc<NonterminalNode> {
        self.file.tree()
    }

    /// Returns a list of all errors encountered during parsing this file.
    pub fn errors(&self) -> &Vec<ParseError> {
        self.file.errors()
    }

    /// Creates a cursor for traversing the syntax tree of this file.
    pub fn create_tree_cursor(&self) -> Cursor {
        self.file.create_tree_cursor()
    }
}

pub struct SemanticAnalysis {
    language_version: Version,
    pub(crate) files: BTreeMap<String, SemanticFile>,
    pub(crate) binder: Binder,
    pub(crate) types: TypeRegistry,
    // TODO(v2): we should obtain the offset/range directly from the AST nodes
    // if they are available
    text_offsets: HashMap<NodeId, TextIndex>,
}

impl SemanticAnalysis {
    pub(crate) fn create<'a>(
        language_version: Version,
        files: impl Iterator<Item = &'a Rc<File>>,
    ) -> Self {
        let mut semantic_analysis = Self::new(language_version);

        for file in files {
            let Some(structured_ast) = passes::p0_build_ast::run_file(file) else {
                // TODO(validation): the file is not valid and cannot be turned
                // into a typed IR tree
                continue;
            };
            let ir_root = passes::p1_flatten_contracts::run_file(
                semantic_analysis.language_version(),
                &structured_ast,
            );
            let semantic_file = SemanticFile {
                file: Rc::clone(file),
                ir_root,
            };
            semantic_analysis.add_file(semantic_file);
        }

        passes::p2_collect_definitions::run(&mut semantic_analysis);
        passes::p3_linearise_contracts::run(&mut semantic_analysis);
        passes::p4_type_definitions::run(&mut semantic_analysis);
        passes::p5_resolve_references::run(&mut semantic_analysis);

        semantic_analysis
    }

    fn new(language_version: Version) -> Self {
        let files = BTreeMap::new();
        let binder = Binder::new();
        let types = TypeRegistry::new(language_version.clone());
        let text_offsets = HashMap::new();

        Self {
            language_version,
            files,
            binder,
            types,
            text_offsets,
        }
    }

    fn add_file(&mut self, file: SemanticFile) {
        // gather text offsets for all non-terminals
        let mut cursor = file.create_tree_cursor();
        self.text_offsets
            .insert(cursor.node().id(), cursor.text_offset());
        while cursor.go_to_next_nonterminal() {
            // find the first non-trivia terminal to register the offset of the
            // non-terminal
            let mut inner = cursor.spawn();
            while inner.go_to_next_terminal() {
                if !inner.node().is_trivia() {
                    break;
                }
            }
            self.text_offsets
                .insert(cursor.node().id(), inner.text_offset());
        }

        // finally add the file to the internal map
        self.files.insert(file.id().to_string(), file);
    }
}

impl SemanticAnalysis {
    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn files(&self) -> Vec<SemanticFile> {
        self.files.values().cloned().collect()
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn binder(&self) -> &Binder {
        &self.binder
    }

    #[cfg(feature = "__private_testing_utils")]
    pub fn types(&self) -> &TypeRegistry {
        &self.types
    }

    pub fn get_file_ast_root(self: &Rc<Self>, file_id: &str) -> Option<ast::SourceUnit> {
        self.files
            .get(file_id)
            .map(|file| create_source_unit(file.ir_root(), self))
    }

    pub(crate) fn node_id_to_file_id(&self, node_id: NodeId) -> Option<String> {
        self.files
            .values()
            .find(|file| file.ir_root.node_id == node_id)
            .map(|file| file.id().to_string())
    }

    pub fn all_definitions(self: &Rc<Self>) -> impl Iterator<Item = Definition> + use<'_> {
        self.binder
            .definitions()
            .values()
            .map(|definition| Definition::create(definition.node_id(), self))
    }

    pub fn find_contract_by_name(self: &Rc<Self>, name: &str) -> Option<ContractDefinition> {
        self.binder
            .definitions()
            .values()
            .find_map(|definition| {
                let binder::Definition::Contract(contract) = definition else {
                    return None;
                };
                if definition.identifier().unparse() == name {
                    Some(contract)
                } else {
                    None
                }
            })
            .map(|contract| create_contract_definition(&contract.ir_node, self))
    }

    // Returns the text offset of the beginning of a non-terminal sequence node.
    // Returns `None` if the information is not available.
    pub(crate) fn get_text_offset_by_node_id(&self, node_id: NodeId) -> Option<TextIndex> {
        self.text_offsets.get(&node_id).copied()
    }

    pub fn get_contracts_abi(self: &Rc<Self>) -> Vec<ContractAbi> {
        let mut contracts = Vec::new();
        for file in self.files.values() {
            let source_unit = create_source_unit(file.ir_root(), self);
            for contract in source_unit.contracts() {
                if contract.abstract_keyword() {
                    continue;
                }
                let contract = contract.abi_with_file_id(file.id());
                contracts.push(contract);
            }
        }
        contracts
    }

    pub fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        self.binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
    }

    pub fn type_canonical_name(&self, type_id: TypeId) -> String {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } => "address".to_string(),
            Type::Array { element_type, .. } => {
                format!(
                    "{element}[]",
                    element = self.type_canonical_name(*element_type)
                )
            }
            Type::Boolean => "bool".to_string(),
            Type::ByteArray { width } => format!("bytes{width}"),
            Type::Bytes { .. } => "bytes".to_string(),
            Type::FixedPointNumber {
                signed,
                bits,
                precision_bits,
            } => format!(
                "{prefix}{bits}x{precision_bits}",
                prefix = if *signed { "fixed" } else { "ufixed" },
            ),
            Type::Function(_) => "function".to_string(),
            Type::Integer { signed, bits } => format!(
                "{prefix}{bits}",
                prefix = if *signed { "int" } else { "uint" }
            ),
            Type::Literal(_) => "literal".to_string(),
            Type::Mapping {
                key_type_id,
                value_type_id,
            } => format!(
                "mapping({key_type} => {value_type})",
                key_type = self.type_canonical_name(*key_type_id),
                value_type = self.type_canonical_name(*value_type_id)
            ),
            Type::String { .. } => "string".to_string(),
            Type::Tuple { types } => format!(
                "({types})",
                types = types
                    .iter()
                    .map(|type_id| self.type_canonical_name(*type_id))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Contract { definition_id }
            | Type::Enum { definition_id }
            | Type::Interface { definition_id }
            | Type::Struct { definition_id, .. }
            | Type::UserDefinedValue { definition_id } => {
                self.definition_canonical_name(*definition_id)
            }
            Type::Void => "void".to_string(),
        }
    }

    const SLOT_SIZE: usize = 32;
    const ADDRESS_BYTE_SIZE: usize = 20;

    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<usize> {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } | Type::Contract { .. } | Type::Interface { .. } => {
                Some(Self::ADDRESS_BYTE_SIZE)
            }
            Type::Boolean => Some(1),
            Type::FixedPointNumber { bits, .. } | Type::Integer { bits, .. } => {
                Some((bits.div_ceil(8)).try_into().unwrap())
            }
            Type::ByteArray { width } => Some((*width).try_into().unwrap()),
            Type::Enum { .. } => Some(1),
            Type::Bytes { .. } | Type::String { .. } => Some(Self::SLOT_SIZE),
            Type::Mapping { .. } => Some(Self::SLOT_SIZE),

            // FIXME: we need to support statically sized arrays properly
            Type::Array { .. } => Some(Self::SLOT_SIZE),

            Type::Function(function_type) => {
                if function_type.external {
                    Some(24)
                } else {
                    // NOTE: an internal function ref type is 8 bytes long, it's
                    // opaque and its meaning not documented
                    Some(8)
                }
            }
            Type::Struct { definition_id, .. } => {
                let binder::Definition::Struct(struct_definition) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                let mut ptr: usize = 0;
                for member in &struct_definition.ir_node.members {
                    let member_type_id = self.binder.node_typing(member.node_id).as_type_id()?;
                    let member_size = self.storage_size_of_type_id(member_type_id)?;
                    let remaining_bytes = Self::SLOT_SIZE - (ptr % Self::SLOT_SIZE);
                    if member_size >= remaining_bytes {
                        ptr += remaining_bytes;
                    }
                    ptr += member_size;
                }
                // round up the final allocation to a full slot, because the
                // next variable needs to start at the next slot anyway
                ptr = ptr.div_ceil(Self::SLOT_SIZE) * Self::SLOT_SIZE;
                Some(ptr)
            }
            Type::UserDefinedValue { definition_id } => {
                let binder::Definition::UserDefinedValueType(user_defined_value) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                self.storage_size_of_type_id(user_defined_value.target_type_id?)
            }

            Type::Literal(_) => None,
            Type::Tuple { .. } => None,
            Type::Void => None,
        }
    }
}
