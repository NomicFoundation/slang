use std::collections::HashSet;

use anyhow::{Error, Result};
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};

use crate::sourcify::Contract;

pub struct CompilationBuilder<'c> {
    internal: InternalCompilationBuilder,
    contract: &'c Contract,
    seen_files: HashSet<String>,
}

impl<'c> CompilationBuilder<'c> {
    pub fn new(contract: &'c Contract) -> Result<CompilationBuilder<'c>> {
        Ok(CompilationBuilder {
            contract,
            internal: InternalCompilationBuilder::create(contract.version.clone())?,
            seen_files: HashSet::new(),
        })
    }

    pub fn build(mut self) -> Result<CompilationUnit> {
        let entrypoint = self.contract.entrypoint().ok_or(Error::msg(format!(
            "Entrypoint not found in contract {name}",
            name = self.contract.name
        )))?;

        self.add_file(&entrypoint)?;

        Ok(self.internal.build())
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        if !self.seen_files.insert(filename.into()) {
            return Ok(());
        }

        let source = self.contract.read_file(filename)?;

        let AddFileResponse { import_paths } = self.internal.add_file(filename.into(), &source);

        for import_path_cursor in import_paths {
            let import_path = import_path_cursor.node().unparse();
            let import_path = import_path
                .strip_prefix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .strip_suffix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .trim();

            let import_real_name = self
                .contract
                .import_resolver
                .resolve_import(filename, import_path)
                .ok_or(Error::msg(format!(
                    "Could not resolve import path {import_path} in source file {filename}"
                )))?;

            self.internal.resolve_import(
                filename,
                &import_path_cursor,
                import_real_name.clone(),
            )?;
            self.add_file(&import_real_name)?;
        }

        Ok(())
    }
}
