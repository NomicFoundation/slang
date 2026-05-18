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
    /// `readme = "README.md"` (or absent).
    Path(String),
    /// `readme = false` — keep the variant so deserialization doesn't fail,
    /// but treat it as "no readme" downstream.
    Disabled(#[allow(dead_code)] bool),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ManifestDependency {
    Simple(String),
    Detailed(DetailedDependency),
}

#[derive(Debug, Default, Deserialize)]
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
        for (target_name, target_table) in self.target {
            let target = Some(target_name);
            collect_dep_table(&target_table.dependencies, "normal", target.clone(), &mut deps);
            collect_dep_table(&target_table.dev_dependencies, "dev", target.clone(), &mut deps);
            collect_dep_table(
                &target_table.build_dependencies,
                "build",
                target.clone(),
                &mut deps,
            );
        }

        let (readme, readme_file) = match self.package.readme {
            Some(ReadmeField::Path(p)) => (Some(p.clone()), Some(p)),
            Some(ReadmeField::Disabled(_)) | None => (None, None),
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
    target: Option<String>,
    out: &mut Vec<RegistryDependency>,
) {
    for (key, dep) in table {
        let (version_req, detailed) = match dep {
            ManifestDependency::Simple(v) => (v.clone(), DetailedDependency::default()),
            ManifestDependency::Detailed(d) => {
                let v = d.version.clone().unwrap_or_else(|| "*".to_owned());
                (v, clone_detailed(d))
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
            target: target.clone(),
            kind: kind.to_owned(),
            registry: detailed.registry,
            explicit_name_in_toml,
        });
    }
}

fn clone_detailed(d: &DetailedDependency) -> DetailedDependency {
    DetailedDependency {
        version: d.version.clone(),
        features: d.features.clone(),
        optional: d.optional,
        default_features: d.default_features,
        registry: d.registry.clone(),
        package: d.package.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inline snippet of a real `.crate`-normalized Cargo.toml — what
    /// `cargo package` writes for `metaslang_cst`. Captured so this test is
    /// self-contained and runs in CI without any prior packaging step.
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
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Cargo.toml");
        std::fs::write(&path, NORMALIZED_FIXTURE).unwrap();

        let md = build_from_packaged_manifest(&path).expect("parse failed");
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
}
