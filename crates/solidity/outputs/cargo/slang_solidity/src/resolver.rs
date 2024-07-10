use std::path::PathBuf;

use metaslang_bindings::PathResolver;

pub struct SolidityPathResolver;

impl PathResolver for SolidityPathResolver {
    fn resolve_path(&self, context_path_str: &str, path_to_resolve_str: &str) -> Option<String> {
        let path_to_resolve = PathBuf::from(path_to_resolve_str);
        if path_to_resolve.is_relative() {
            PathBuf::from(context_path_str)
                .parent()
                .map(|parent| parent.join(path_to_resolve))
                .and_then(|resolved| resolved.canonicalize().ok())
                .map(|resolved| resolved.to_string_lossy().to_string())
        } else {
            // TODO: for file paths this needs to try to apply a base path, and
            // then all include paths in order
            Some(path_to_resolve_str.to_string())
        }
    }
}
