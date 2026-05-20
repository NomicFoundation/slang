//! Build the registry-publish JSON payload that crates.io's `/api/v1/crates/new`
//! endpoint expects.
//!
//! Format reference: <https://doc.rust-lang.org/cargo/reference/registries.html#publish>
//!
//! We construct this in the prepare step from the rewritten `Cargo.toml` that
//! `cargo package` produces (workspace inheritance resolved, path deps stripped),
//! and ship the resulting JSON alongside the `.crate` in `target/publish-artifacts/`.
//! The publish step is then a pure download-and-POST — no cargo binary involved.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RegistryMetadata {
    pub name: String,
    pub vers: String,
    pub deps: Vec<RegistryDependency>,
    pub features: BTreeMap<String, Vec<String>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
    pub readme_file: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub badges: BTreeMap<String, BTreeMap<String, String>>,
    pub links: Option<String>,
    pub rust_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegistryDependency {
    pub name: String,
    pub version_req: String,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    pub target: Option<String>,
    pub kind: String,
    pub registry: Option<String>,
    pub explicit_name_in_toml: Option<String>,
}

/// Read the Cargo.toml that `cargo package` wrote into the extracted package
/// directory. After `cargo package`, workspace inheritance is resolved and
/// path-deps are rewritten to version-only specs — exactly what crates.io needs.
pub fn build_from_packaged_manifest(manifest_path: &Path) -> Result<RegistryMetadata> {
    let contents = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read packaged Cargo.toml: {manifest_path:?}"))?;
    let parsed: PackagedManifest = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse packaged Cargo.toml: {manifest_path:?}"))?;
    Ok(parsed.into_registry_metadata())
}

#[derive(Debug, Deserialize)]
struct PackagedManifest {
    package: PackageTable,
    #[serde(default)]
    dependencies: BTreeMap<String, ManifestDependency>,
    #[serde(default, rename = "dev-dependencies")]
    dev_dependencies: BTreeMap<String, ManifestDependency>,
    #[serde(default, rename = "build-dependencies")]
    build_dependencies: BTreeMap<String, ManifestDependency>,
    #[serde(default)]
    target: BTreeMap<String, TargetTable>,
    #[serde(default)]
    features: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
struct TargetTable {
    #[serde(default)]
    dependencies: BTreeMap<String, ManifestDependency>,
    #[serde(default, rename = "dev-dependencies")]
    dev_dependencies: BTreeMap<String, ManifestDependency>,
    #[serde(default, rename = "build-dependencies")]
    build_dependencies: BTreeMap<String, ManifestDependency>,
}

#[derive(Debug, Deserialize)]
struct PackageTable {
    name: String,
    version: String,
    #[serde(default)]
    authors: Vec<String>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    readme: Option<ReadmeField>,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default)]
    categories: Vec<String>,
    license: Option<String>,
    license_file: Option<String>,
    repository: Option<String>,
    links: Option<String>,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ReadmeField {
    /// `readme = "README.md"`.
    Path(String),
    /// `readme = false` — boolean form is allowed in TOML; we treat any value
    /// downstream as "no readme", but we match against the bool so the field
    /// isn't dead code.
    Disabled(bool),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ManifestDependency {
    Simple(String),
    Detailed(DetailedDependency),
}

#[derive(Clone, Debug, Default, Deserialize)]
struct DetailedDependency {
    version: Option<String>,
    #[serde(default)]
    features: Vec<String>,
    #[serde(default)]
    optional: bool,
    #[serde(default = "default_true", rename = "default-features")]
    default_features: bool,
    registry: Option<String>,
    package: Option<String>,
}

fn default_true() -> bool {
    true
}

impl PackagedManifest {
    fn into_registry_metadata(self) -> RegistryMetadata {
        let mut deps = vec![];
        collect_dep_table(&self.dependencies, "normal", None, &mut deps);
        collect_dep_table(&self.dev_dependencies, "dev", None, &mut deps);
        collect_dep_table(&self.build_dependencies, "build", None, &mut deps);
        for (target_name, target_table) in &self.target {
            let target = Some(target_name.as_str());
            collect_dep_table(&target_table.dependencies, "normal", target, &mut deps);
            collect_dep_table(&target_table.dev_dependencies, "dev", target, &mut deps);
            collect_dep_table(&target_table.build_dependencies, "build", target, &mut deps);
        }

        let (readme, readme_file) = match self.package.readme {
            Some(ReadmeField::Path(p)) => (Some(p.clone()), Some(p)),
            Some(ReadmeField::Disabled(false)) | None => (None, None),
            // `readme = true` is not a valid TOML manifest shape — fail loudly
            // rather than silently emitting weird registry metadata.
            Some(ReadmeField::Disabled(true)) => {
                panic!("readme = true is not a valid Cargo.toml shape")
            }
        };

        RegistryMetadata {
            name: self.package.name,
            vers: self.package.version,
            deps,
            features: self.features,
            authors: self.package.authors,
            description: self.package.description,
            documentation: self.package.documentation,
            homepage: self.package.homepage,
            readme,
            readme_file,
            keywords: self.package.keywords,
            categories: self.package.categories,
            license: self.package.license,
            license_file: self.package.license_file,
            repository: self.package.repository,
            badges: BTreeMap::new(),
            links: self.package.links,
            rust_version: self.package.rust_version,
        }
    }
}

fn collect_dep_table(
    table: &BTreeMap<String, ManifestDependency>,
    kind: &str,
    target: Option<&str>,
    out: &mut Vec<RegistryDependency>,
) {
    for (key, dep) in table {
        let (version_req, detailed) = match dep {
            ManifestDependency::Simple(v) => (v.clone(), DetailedDependency::default()),
            ManifestDependency::Detailed(d) => {
                let v = d.version.clone().unwrap_or_else(|| "*".to_owned());
                (v, d.clone())
            }
        };
        // If the manifest renames the dep via `package = "real-name"`, the registry
        // entry uses the real name and `explicit_name_in_toml` carries the alias.
        let (name, explicit_name_in_toml) = match &detailed.package {
            Some(real) => (real.clone(), Some(key.clone())),
            None => (key.clone(), None),
        };
        out.push(RegistryDependency {
            name,
            version_req,
            features: detailed.features,
            optional: detailed.optional,
            default_features: detailed.default_features,
            target: target.map(str::to_owned),
            kind: kind.to_owned(),
            registry: detailed.registry,
            explicit_name_in_toml,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(toml_str: &str) -> RegistryMetadata {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Cargo.toml");
        std::fs::write(&path, toml_str).unwrap();
        build_from_packaged_manifest(&path).expect("parse failed")
    }

    /// Real `.crate`-normalized `Cargo.toml` for `metaslang_cst` — captured
    /// from `cargo package --no-verify --package metaslang_cst`. If `cargo`
    /// changes its serializer in a way that breaks our parser, this fixture
    /// will go stale; re-extract by running:
    ///
    ///     ./bin/cargo package --no-verify --allow-dirty --package metaslang_cst
    ///     tar -xzOf target/package/metaslang_cst-*.crate metaslang_cst-*/Cargo.toml
    const NORMALIZED_FIXTURE: &str = r#"
[package]
edition = "2021"
rust-version = "1.94.0"
name = "metaslang_cst"
version = "1.3.5"
authors = [
    "Slang Team <slang@nomic.foundation>",
    "Nomic Foundation <packages@nomic.foundation>",
]
description = "Generic CST library."
homepage = "https://nomicfoundation.github.io/slang/"
readme = "README.md"
keywords = ["parser"]
categories = ["compilers", "parsing", "parser-implementations"]
license = "MIT"
repository = "https://github.com/NomicFoundation/slang/"

[dependencies.nom]
version = "8.0.0"

[dependencies.serde]
version = "1.0.219"
features = ["derive", "rc"]

[dependencies.thiserror]
version = "2.0.12"
"#;

    #[test]
    fn parses_normalized_manifest_shape() {
        let md = parse(NORMALIZED_FIXTURE);
        assert_eq!(md.name, "metaslang_cst");
        assert_eq!(md.vers, "1.3.5");
        assert_eq!(md.license.as_deref(), Some("MIT"));
        assert_eq!(md.readme.as_deref(), Some("README.md"));
        assert_eq!(md.readme_file.as_deref(), Some("README.md"));
        assert_eq!(md.rust_version.as_deref(), Some("1.94.0"));

        let serde_dep = md
            .deps
            .iter()
            .find(|d| d.name == "serde")
            .expect("serde dep present");
        assert_eq!(serde_dep.kind, "normal");
        assert_eq!(serde_dep.version_req, "1.0.219");
        assert!(serde_dep.features.contains(&"derive".to_string()));
        assert!(serde_dep.features.contains(&"rc".to_string()));
        assert!(serde_dep.default_features);
        assert!(!serde_dep.optional);

        let nom_dep = md.deps.iter().find(|d| d.name == "nom").unwrap();
        assert_eq!(nom_dep.version_req, "8.0.0");
    }

    /// TOML treats `[dependencies.foo]` and `[dependencies] foo = { ... }` as
    /// the same structure, so `cargo package`'s subtable form and any
    /// hypothetical inline-table form deserialize identically. Lock that in.
    #[test]
    fn inline_table_dep_form_parses_identically() {
        let inline = r#"
[package]
name = "x"
version = "0.1.0"

[dependencies]
serde = { version = "1.0.219", features = ["derive"], default-features = false }
"#;
        let md = parse(inline);
        let dep = md.deps.iter().find(|d| d.name == "serde").unwrap();
        assert_eq!(dep.version_req, "1.0.219");
        assert!(dep.features.contains(&"derive".to_string()));
        assert!(!dep.default_features);
    }

    /// dev- and build-dependencies must each be tagged with the right `kind`.
    #[test]
    fn dev_and_build_dependencies_get_distinct_kinds() {
        let toml = r#"
[package]
name = "x"
version = "0.1.0"

[dependencies]
runtime_dep = "1"

[dev-dependencies]
test_dep = "2"

[build-dependencies]
build_dep = "3"
"#;
        let md = parse(toml);
        let by_name = |n: &str| md.deps.iter().find(|d| d.name == n).expect(n);
        assert_eq!(by_name("runtime_dep").kind, "normal");
        assert_eq!(by_name("test_dep").kind, "dev");
        assert_eq!(by_name("build_dep").kind, "build");
    }

    /// `[target.'cfg(unix)'.dependencies]` deps must carry the target string.
    #[test]
    fn target_specific_dependencies_carry_target() {
        let toml = r#"
[package]
name = "x"
version = "0.1.0"

[target."cfg(unix)".dependencies]
unix_only = "1"

[target."cfg(windows)".dev-dependencies]
windows_test = "2"
"#;
        let md = parse(toml);
        let unix = md.deps.iter().find(|d| d.name == "unix_only").unwrap();
        assert_eq!(unix.target.as_deref(), Some("cfg(unix)"));
        assert_eq!(unix.kind, "normal");
        let win = md.deps.iter().find(|d| d.name == "windows_test").unwrap();
        assert_eq!(win.target.as_deref(), Some("cfg(windows)"));
        assert_eq!(win.kind, "dev");
    }

    /// `key = { package = "real-name", ... }` is a rename: the registry entry
    /// uses the real crate name; `explicit_name_in_toml` carries the alias.
    #[test]
    fn package_rename_uses_explicit_name_in_toml() {
        let toml = r#"
[package]
name = "x"
version = "0.1.0"

[dependencies.aliased]
version = "1"
package = "real_crate"
"#;
        let md = parse(toml);
        let dep = md.deps.iter().find(|d| d.name == "real_crate").unwrap();
        assert_eq!(dep.explicit_name_in_toml.as_deref(), Some("aliased"));
    }

    /// `features` with `dep:` and `?/` syntax must pass through verbatim.
    /// crates.io interprets these; we just ferry them.
    #[test]
    fn features_with_dep_syntax_pass_through() {
        let toml = r#"
[package]
name = "x"
version = "0.1.0"

[dependencies.optional_dep]
version = "1"
optional = true

[features]
default = []
extra = ["dep:optional_dep", "optional_dep?/std"]
"#;
        let md = parse(toml);
        let extra = md.features.get("extra").expect("extra feature present");
        assert!(extra.contains(&"dep:optional_dep".to_string()));
        assert!(extra.contains(&"optional_dep?/std".to_string()));

        let dep = md.deps.iter().find(|d| d.name == "optional_dep").unwrap();
        assert!(dep.optional);
    }

    /// `readme = false` (disabling the README) must not produce a
    /// `readme`/`readme_file` field in the registry JSON.
    #[test]
    fn readme_disabled_yields_no_readme_fields() {
        let toml = r#"
[package]
name = "x"
version = "0.1.0"
readme = false
"#;
        let md = parse(toml);
        assert!(md.readme.is_none());
        assert!(md.readme_file.is_none());
    }

    /// Live round-trip: run `cargo package --no-verify` on a real slang crate,
    /// extract the normalized `Cargo.toml` from the resulting `.crate`, and
    /// feed it through our parser. Catches the moment cargo's serializer
    /// changes a shape we don't understand.
    ///
    /// Marked `#[ignore]` because it shells out to cargo (which is heavy and
    /// fragile under workspace lock contention). Run on demand:
    ///
    ///     ./bin/cargo test -p infra_cli -- --ignored metadata::tests::round_trip
    #[test]
    #[ignore = "shells out to cargo package; run with --ignored"]
    fn round_trip_real_cargo_package() {
        use std::process::Command;
        let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
            .map(std::path::PathBuf::from)
            .expect("CARGO_MANIFEST_DIR")
            .ancestors()
            .nth(3)
            .expect("workspace root")
            .to_path_buf();

        let status = Command::new("cargo")
            .current_dir(&workspace_root)
            .args([
                "package",
                "--no-verify",
                "--allow-dirty",
                "--package",
                "metaslang_cst",
            ])
            .status()
            .expect("cargo package should run");
        assert!(status.success(), "cargo package failed");

        let pkg_dir = workspace_root.join("target/package");
        let crate_file = std::fs::read_dir(&pkg_dir)
            .expect("read target/package")
            .filter_map(Result::ok)
            .map(|e| e.path())
            .find(|p| {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let ext_is_crate = p
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("crate"));
                name.starts_with("metaslang_cst-") && ext_is_crate
            })
            .expect("expected metaslang_cst .crate");

        // Reuse the extraction primitive a real publish would use.
        let file = std::fs::File::open(&crate_file).unwrap();
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
        let dir = tempfile::tempdir().unwrap();
        let manifest_path = dir.path().join("Cargo.toml");
        let mut found = false;
        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap().into_owned();
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let parent_name = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if filename == "Cargo.toml" && parent_name.starts_with("metaslang_cst-") {
                let mut contents = Vec::new();
                std::io::copy(&mut entry, &mut contents).unwrap();
                std::fs::write(&manifest_path, contents).unwrap();
                found = true;
                break;
            }
        }
        assert!(found, "Cargo.toml not found inside .crate");

        let md = build_from_packaged_manifest(&manifest_path).expect("parse should succeed");
        assert_eq!(md.name, "metaslang_cst");
        assert!(!md.deps.is_empty(), "expected metaslang_cst to have deps");
    }
}
