//! Build the registry-publish JSON payload that crates.io's `/api/v1/crates/new`
//! endpoint expects.
//!
//! We parse the rewritten `Cargo.toml` that `cargo package` produces (workspace
//! inheritance resolved, path deps stripped) with `cargo-manifest`, then convert
//! to `crates_io::NewCrate` — the official, version-tracked schema the cargo
//! team ships in `rust-lang/cargo`. The on-disk JSON is whatever
//! `serde_json::to_vec_pretty(&NewCrate)` produces, so it tracks any future
//! schema additions automatically.
//!
//! Format reference: <https://doc.rust-lang.org/cargo/reference/registries.html#publish>
use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{bail, Context, Result};
use cargo_manifest::{Dependency, DepsSet, Manifest, MaybeInherited, Package, StringOrBool};
use crates_io::{NewCrate, NewCrateDependency};

pub fn build_new_crate(packaged_manifest_path: &Path) -> Result<NewCrate> {
    let manifest = Manifest::from_path(packaged_manifest_path).with_context(|| {
        format!("Failed to parse packaged Cargo.toml: {packaged_manifest_path:?}")
    })?;
    let package = manifest
        .package
        .with_context(|| format!("Missing [package] in {packaged_manifest_path:?}"))?;

    let mut deps = vec![];
    if let Some(d) = &manifest.dependencies {
        collect_deps(d, "normal", None, &mut deps)?;
    }
    if let Some(d) = &manifest.dev_dependencies {
        collect_deps(d, "dev", None, &mut deps)?;
    }
    if let Some(d) = &manifest.build_dependencies {
        collect_deps(d, "build", None, &mut deps)?;
    }
    if let Some(targets) = &manifest.target {
        for (target_name, target) in targets {
            let t = Some(target_name.as_str());
            collect_deps(&target.dependencies, "normal", t, &mut deps)?;
            collect_deps(&target.dev_dependencies, "dev", t, &mut deps)?;
            collect_deps(&target.build_dependencies, "build", t, &mut deps)?;
        }
    }

    let (readme, readme_file) = extract_readme(&package);

    Ok(NewCrate {
        name: package.name,
        vers: resolve_inherited(package.version)
            .context("package.version missing from packaged manifest")?,
        deps,
        features: manifest.features.unwrap_or_default(),
        authors: resolve_inherited(package.authors).unwrap_or_default(),
        description: resolve_inherited(package.description),
        documentation: resolve_inherited(package.documentation),
        homepage: resolve_inherited(package.homepage),
        readme,
        readme_file,
        keywords: resolve_inherited(package.keywords).unwrap_or_default(),
        categories: resolve_inherited(package.categories).unwrap_or_default(),
        license: resolve_inherited(package.license),
        license_file: resolve_inherited(package.license_file),
        repository: resolve_inherited(package.repository),
        // `badges` is a deprecated crates.io concept; cargo-manifest exposes it
        // as `Option<Badges>` (a typed wrapper), but the registry API expects
        // a freeform string-map-of-string-maps. Emit an empty map to match
        // what `cargo publish` does for crates that don't use badges.
        badges: BTreeMap::new(),
        links: package.links,
        rust_version: resolve_inherited(package.rust_version),
    })
}

/// `cargo package` resolves workspace inheritance in the manifest it writes, so
/// every field arrives here as `MaybeInherited::Local`. If we see an
/// `Inherited` variant in a packaged manifest, cargo's packaging is broken —
/// bail rather than silently drop the value.
fn resolve_inherited<T>(value: Option<MaybeInherited<T>>) -> Option<T> {
    match value {
        Some(MaybeInherited::Local(v)) => Some(v),
        Some(MaybeInherited::Inherited { .. }) => {
            panic!("Packaged manifest contains an unresolved workspace-inherited field")
        }
        None => None,
    }
}

fn extract_readme<M>(package: &Package<M>) -> (Option<String>, Option<String>) {
    match &package.readme {
        Some(MaybeInherited::Local(StringOrBool::String(p))) => (Some(p.clone()), Some(p.clone())),
        Some(MaybeInherited::Local(StringOrBool::Bool(false))) | None => (None, None),
        Some(MaybeInherited::Local(StringOrBool::Bool(true))) => {
            // `readme = true` isn't a valid Cargo.toml shape.
            panic!("readme = true is not valid in Cargo.toml")
        }
        Some(MaybeInherited::Inherited { .. }) => {
            panic!("Packaged manifest contains an unresolved workspace-inherited readme")
        }
    }
}

fn collect_deps(
    table: &DepsSet,
    kind: &str,
    target: Option<&str>,
    out: &mut Vec<NewCrateDependency>,
) -> Result<()> {
    for (key, dep) in table {
        out.push(convert_dep(key, dep, kind, target)?);
    }
    Ok(())
}

fn convert_dep(
    key: &str,
    dep: &Dependency,
    kind: &str,
    target: Option<&str>,
) -> Result<NewCrateDependency> {
    let dep = match dep {
        Dependency::Simple(version) => NewCrateDependency {
            optional: false,
            default_features: true,
            name: key.to_owned(),
            features: vec![],
            version_req: version.clone(),
            target: target.map(str::to_owned),
            kind: kind.to_owned(),
            registry: None,
            explicit_name_in_toml: None,
            artifact: None,
            bindep_target: None,
            lib: false,
        },
        Dependency::Detailed(detail) => {
            // `key = { package = "real" }` renames a dep: registry sees the real
            // crate name, the alias goes in `explicit_name_in_toml`.
            let (name, explicit_name_in_toml) = match &detail.package {
                Some(real) => (real.clone(), Some(key.to_owned())),
                None => (key.to_owned(), None),
            };
            NewCrateDependency {
                optional: detail.optional.unwrap_or(false),
                default_features: detail.default_features.unwrap_or(true),
                name,
                features: detail.features.clone().unwrap_or_default(),
                version_req: detail.version.clone().unwrap_or_else(|| "*".to_owned()),
                target: target.map(str::to_owned),
                kind: kind.to_owned(),
                registry: detail.registry.clone(),
                explicit_name_in_toml,
                artifact: None,
                bindep_target: None,
                lib: false,
            }
        }
        Dependency::Inherited(_) => {
            bail!(
                "Dependency {key:?} is still workspace-inherited in packaged manifest; \
                 cargo should have resolved it"
            )
        }
    };
    Ok(dep)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(toml_str: &str) -> NewCrate {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Cargo.toml");
        std::fs::write(&path, toml_str).unwrap();
        build_new_crate(&path).expect("parse failed")
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

        let md = build_new_crate(&manifest_path).expect("parse should succeed");
        assert_eq!(md.name, "metaslang_cst");
        assert!(!md.deps.is_empty(), "expected metaslang_cst to have deps");
    }
}
