use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult, Position},
};
use serde::de::DeserializeOwned;

pub fn deserialize_yaml<T: DeserializeOwned>(
    codegen: &mut CodegenContext,
    file_path: &PathBuf,
) -> CodegenResult<T> {
    let source = codegen.read_file(file_path).unwrap();

    return serde_yaml::from_str(&source).map_err(|error| {
        let location = error.location().unwrap();
        let start = Position::new(location.index(), location.line(), location.column());

        let width = source.lines().nth(location.line() - 1).unwrap().len() - start.column;
        let end = Position::new(start.offset + width, start.line, start.column + width);

        let range = start..end;
        return CodegenErrors::single(file_path.to_owned(), range, error.to_string());
    });
}
