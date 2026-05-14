use std::collections::BTreeMap;
use std::sync::Arc;

use slang_solidity_v2_ast::{abi, ast};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use super::file::InternalFile;
use super::{File, FileStruct};

pub struct CompilationUnit {
    language_version: LanguageVersion,
    files: BTreeMap<String, Arc<InternalFile>>,
    semantic: Arc<SemanticContext>,
    diagnostics: DiagnosticCollection,
}

impl CompilationUnit {
    pub(super) fn create(
        language_version: LanguageVersion,
        files: Vec<InternalFile>,
        semantic: SemanticContext,
        diagnostics: DiagnosticCollection,
    ) -> Self {
        let files: BTreeMap<String, Arc<InternalFile>> = files
            .into_iter()
            .map(|file| (file.id().to_string(), Arc::new(file)))
            .collect();
        Self {
            language_version,
            files,
            semantic: Arc::new(semantic),
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

    /// Returns a list of all file IDs in the compilation unit.
    pub fn file_ids(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    pub fn iter_files(&self) -> impl Iterator<Item = File> + use<'_> {
        self.files
            .values()
            .map(|internal_file| FileStruct::create(internal_file, &self.semantic))
    }

    pub fn get_file_by_id(&self, id: &str) -> Option<File> {
        self.files
            .get(id)
            .map(|internal_file| FileStruct::create(internal_file, &self.semantic))
    }

    #[cfg(feature = "__private_testing_utils")]
    #[doc(hidden)]
    pub fn semantic(&self) -> &Arc<SemanticContext> {
        &self.semantic
    }

    #[cfg(feature = "__private_testing_utils")]
    #[doc(hidden)]
    pub fn get_file_ir_root(&self, file_id: &str) -> Option<slang_solidity_v2_ir::ir::SourceUnit> {
        self.files
            .get(file_id)
            .map(|file| Arc::clone(file.ir_root()))
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

    pub fn find_contract_by_name(&self, name: &str) -> Option<ast::ContractDefinition> {
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
