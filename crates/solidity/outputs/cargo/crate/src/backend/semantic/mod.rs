use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use semver::Version;

use self::ast::{create_contract_definition, create_source_unit, ContractDefinition, Definition};
use crate::backend::binder::Binder;
pub use crate::backend::ir::{ast, ir2_flat_contracts as output_ir};
use crate::backend::types::TypeRegistry;
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
}
