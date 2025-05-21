use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

/// Resolves an import of a solidity file. Parameters are:
/// - `directory`: the directory of the solidity project,
/// - `source_file`: the relative path to the file under inspection,
/// - `import_string`: the import string as parsed from the source file.
///
/// Returns the relative path of the imported file.
pub fn resolve_import(
    directory: &PathBuf,
    source_file: &str,
    import_string: &str,
) -> Result<String> {
    let source_file_dir = Path::new(source_file)
        .parent()
        .ok_or_else(|| anyhow!("Invalid source file path"))?;

    // Sanitize the import string: remove leading slashes
    let sanitized_import = import_string.trim_start_matches('/');

    // First attempt: resolve relative to the source file directory
    let file = source_file_dir.join(sanitized_import);
    let real_file = Path::new(directory).join(&file).canonicalize();

    if let Ok(real_file) = real_file {
        if real_file.exists() {
            return Ok(real_file.to_string_lossy().to_string());
        }
    }

    // Second attempt: resolve relative to the directory
    let real_file = Path::new(directory).join(sanitized_import).canonicalize();
    if let Ok(real_file) = real_file {
        if real_file.exists() {
            return Ok(real_file.to_string_lossy().to_string());
        }
    }

    // If neither attempt succeeds, throw an error
    Err(anyhow!(
        "Can't resolve import {import_string} in the context of {source_file_dir:?}"
    ))
}
