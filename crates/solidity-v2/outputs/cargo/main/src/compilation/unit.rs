use std::collections::BTreeMap;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_semantic::binder::Binder;
use slang_solidity_v2_semantic::compilation::file::File;
use slang_solidity_v2_semantic::types::TypeRegistry;

pub struct CompilationUnit {
    language_version: LanguageVersion,
    files: BTreeMap<String, Rc<File>>,
    binder: Binder,
    types: TypeRegistry,
}

impl CompilationUnit {
    pub(super) fn create(
        language_version: LanguageVersion,
        files: Vec<File>,
        binder: Binder,
        types: TypeRegistry,
    ) -> Self {
        let files: BTreeMap<String, Rc<File>> = files
            .into_iter()
            .map(|file| (file.id().to_string(), Rc::new(file)))
            .collect();
        Self {
            language_version,
            files,
            binder,
            types,
        }
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

    // TODO: this should not be public
    pub fn binder(&self) -> &Binder {
        &self.binder
    }

    // TODO: this should not be public
    pub fn types(&self) -> &TypeRegistry {
        &self.types
    }
}
