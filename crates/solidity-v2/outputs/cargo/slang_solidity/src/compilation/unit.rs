use std::collections::BTreeMap;
use std::rc::Rc;

use slang_solidity_v2_ast::{abi, ast};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use super::file::File;

pub struct CompilationUnit {
    language_version: LanguageVersion,
    files: BTreeMap<String, Rc<File>>,
    semantic: Rc<SemanticContext>,
    diagnostics: DiagnosticCollection,
}

impl CompilationUnit {
    pub(super) fn create(
        language_version: LanguageVersion,
        files: Vec<File>,
        semantic: Rc<SemanticContext>,
        diagnostics: DiagnosticCollection,
    ) -> Self {
        let files: BTreeMap<String, Rc<File>> = files
            .into_iter()
            .map(|file| (file.id().to_string(), Rc::new(file)))
            .collect();
        Self {
            language_version,
            files,
            semantic,
            diagnostics,
        }
    }

    /// Returns the diagnostics collected during the compilation.
    pub fn diagnostics(&self) -> &DiagnosticCollection {
        &self.diagnostics
    }

    /// Returns the language version this compilation unit is configured for.
    pub fn language_version(&self) -> LanguageVersion {
        self.language_version
    }

    /// Returns a list of all files in the compilation unit.
    pub fn files(&self) -> Vec<Rc<File>> {
        self.files.values().cloned().collect()
    }

    /// Returns the file with the specified ID, if it exists.
    pub fn file(&self, id: &str) -> Option<Rc<File>> {
        self.files.get(id).cloned()
    }

    // TODO: this should be semi-public
    pub fn semantic(&self) -> &Rc<SemanticContext> {
        &self.semantic
    }

    pub fn get_file_ast_root(&self, file_id: &str) -> Option<ast::SourceUnit> {
        self.files
            .get(file_id)
            .map(|file| ast::create_source_unit(file.ir_root(), &self.semantic))
    }

    pub fn all_definitions(&self) -> impl Iterator<Item = ast::Definition> + use<'_> {
        self.semantic.all_definitions().map(|definition| {
            ast::Definition::try_create(definition.node_id(), &self.semantic).unwrap()
        })
    }

    pub fn all_references(&self) -> impl Iterator<Item = ast::Reference> + use<'_> {
        self.semantic.all_references().map(|reference| {
            ast::Reference::try_create(&reference.identifier, &self.semantic).unwrap()
        })
    }

    pub fn find_contract_by_name(self: &Rc<Self>, name: &str) -> Option<ast::ContractDefinition> {
        self.semantic
            .find_contract_by_name(name)
            .map(|contract| ast::create_contract_definition(&contract, &self.semantic))
    }

    pub fn compute_contracts_abi(&self) -> Vec<abi::ContractAbi> {
        self.files
            .values()
            .flat_map(|file| {
                ast::create_source_unit(file.ir_root(), &self.semantic).compute_contracts_abi()
            })
            .collect()
    }
}
