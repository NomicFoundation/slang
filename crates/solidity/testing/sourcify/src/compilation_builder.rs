use anyhow::Result;
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};

use crate::sourcify::Contract;

pub struct CompilationBuilder<'c> {
    internal: InternalCompilationBuilder,
    contract: &'c Contract,
    seen_files: Vec<String>,
    read_buffer: String,
}

impl <'c> CompilationBuilder<'c> {
    pub fn new(contract: &'c Contract) -> Result<CompilationBuilder<'c>> {
        Ok(CompilationBuilder {
            contract,
            internal: InternalCompilationBuilder::create(contract.version())?,
            seen_files: vec![],
            read_buffer: String::new(),
        })
    }

    pub fn create_compilation_unit(&mut self) -> Result<CompilationUnit> {
        let entrypoint = self.contract.entrypoint()?;

        self.add_file(&entrypoint)?;

        Ok(self.internal.build())
    }

    fn add_file(&mut self, filename: &str) -> Result<()> {
        let filename: String = filename.into();
        if self.seen_files.contains(&filename) {
            return Ok(());
        }

        self.read_buffer.clear();
        self.seen_files.push(filename.clone());

        self.contract.read_file(&filename, &mut self.read_buffer)?;

        let AddFileResponse { import_paths } = self.internal.add_file(filename.clone(), &self.read_buffer);

        for import_path in import_paths {
            let import_path = import_path.node().unparse();
            let import_path = import_path
                .strip_prefix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .strip_suffix(|c| matches!(c, '"' | '\''))
                .unwrap()
                .trim();


            let import_real_name = self.contract.metadata.resolve_import(&filename, &import_path)?;
            self.add_file(&import_real_name)?;
        }

        Ok(())
    }
}
