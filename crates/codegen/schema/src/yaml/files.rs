use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult, Position},
};
use serde::de::DeserializeOwned;

use crate::yaml::{cst, parser::Parser};

pub struct File<T> {
    pub path: std::path::PathBuf,
    pub cst_node: cst::NodeRef,
    pub value: T,
}

impl<T: DeserializeOwned> File<T> {
    pub fn load(codegen: &mut CodegenContext, path: PathBuf) -> CodegenResult<Self> {
        let source = &codegen.read_file(&path).unwrap();

        let cst_node = Parser::run_parser(&path, source)?;

        let value: T = serde_yaml::from_str(source).map_err(|error| {
            let range = {
                let location = error.location().unwrap();
                let position = Position::new(location.index(), location.line(), location.column());
                cst_node
                    .pinpoint(&position)
                    .map_or_else(|| position..position, |node| node.range().to_owned())
            };

            return CodegenErrors::single(&path, &range, Errors::Serde(error));
        })?;

        return Ok(File {
            path,
            cst_node,
            value,
        });
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("{0}")]
    Serde(serde_yaml::Error),
}
