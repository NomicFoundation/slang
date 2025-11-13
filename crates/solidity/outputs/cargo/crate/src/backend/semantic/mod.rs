use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::backend::binder::Binder;
pub use crate::backend::ir::ir2_flat_contracts as output_ir;
use crate::backend::passes;
use crate::backend::types::TypeRegistry;
use crate::compilation::File;
use crate::cst::{Cursor, NonterminalNode};
use crate::parser::ParseError;

pub struct SemanticBuilder {
    language_version: Version,
    files: Vec<Rc<File>>,
}

impl SemanticBuilder {
    pub fn create(language_version: Version) -> Self {
        Self {
            language_version,
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: Rc<File>) {
        self.files.push(file);
    }

    pub fn build(self) -> SemanticAnalysis {
        SemanticAnalysis::create(self.language_version, self.files)
    }
}

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

    pub fn ir_root(&self) -> &output_ir::SourceUnit {
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
}

impl SemanticAnalysis {
    fn create(language_version: Version, files: Vec<Rc<File>>) -> Self {
        let mut semantic_analysis = Self::new(language_version);

        for file in files {
            let file_id = file.id().to_string();
            let Some(structured_ast) = passes::p0_build_ast::run_file(&file) else {
                // TODO(validation): the file is not valid and cannot be turned
                // into a typed IR tree
                continue;
            };
            let ir_root = passes::p1_flatten_contracts::run_file(
                semantic_analysis.language_version(),
                &structured_ast,
            );
            let semantic_file = SemanticFile { file, ir_root };
            semantic_analysis.files.insert(file_id, semantic_file);
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

        Self {
            language_version,
            files,
            binder,
            types,
        }
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
}
