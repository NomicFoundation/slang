use std::collections::HashSet;

use anyhow::Result;
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};

use crate::sourcify::Contract;

pub struct CompilationBuilder<'c> {
    internal: InternalCompilationBuilder,
    contract: &'c Contract,
    seen_files: HashSet<String>,
    read_buffer: String,
}

impl<'c> CompilationBuilder<'c> {
    pub fn new(contract: &'c Contract) -> Result<CompilationBuilder<'c>> {
        Ok(CompilationBuilder {
            contract,
            internal: InternalCompilationBuilder::create(contract.version.clone())?,
            seen_files: HashSet::new(),
            read_buffer: String::new(),
        })
    }

    pub fn build(mut self) -> Result<CompilationUnit> {
        let entrypoint = self.contract.entrypoint()?;

        self.add_file(&entrypoint)?;

        Ok(self.internal.build())
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        if !self.seen_files.insert(filename.into()) {
            return Ok(());
        }

        self.read_buffer.clear();

        self.contract.read_file(&filename, &mut self.read_buffer)?;

        let AddFileResponse { import_paths } =
            self.internal.add_file(filename.into(), &self.read_buffer);

        for import_path in import_paths {
            let import_path = import_path.node().unparse();
            let import_path = import_path
                .strip_prefix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .strip_suffix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .trim();

            let import_real_name = self
                .contract
                .import_resolver
                .resolve_import(&filename, import_path)?;
            self.add_file(&import_real_name)?;
        }

        Ok(())
    }
}
