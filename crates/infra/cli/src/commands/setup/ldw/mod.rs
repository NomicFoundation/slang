use std::path::Path;

use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

/// Install npm packages for ldw.
pub fn setup_ldw() {
    Command::new("npm")
        .current_dir(Path::repo_path("crates/codegen/ldw"))
        .arg("install")
        .run();
}
