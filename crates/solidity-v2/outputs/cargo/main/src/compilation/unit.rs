use std::collections::BTreeMap;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_semantic::context::{SemanticFile, SemanticContext};

use super::file::File;

pub struct CompilationUnit {
    language_version: LanguageVersion,
    files: BTreeMap<String, Rc<File>>,
    semantic: Rc<SemanticContext>,
}

impl CompilationUnit {
    pub(super) fn create(
        language_version: LanguageVersion,
        files: Vec<File>,
        semantic: Rc<SemanticContext>,
    ) -> Self {
        let files: BTreeMap<String, Rc<File>> = files
            .into_iter()
            .map(|file| (file.id().to_string(), Rc::new(file)))
            .collect();
        Self {
            language_version,
            files,
            semantic,
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

    pub fn semantic(&self) -> &Rc<SemanticContext> {
        &self.semantic
    }
}
