use std::path::Path;

use anyhow::{bail, Result};
use infra_utils::commands::Command;
use semver::Version;
use serde::de;

use crate::toolchains::napi::{BuildTarget, NapiConfig, NapiResolver};

/// A GLIBC version (e.g. "2.16") supported by Zig(build) for cross-compilation.
///
/// See the list of supported versions at <https://github.com/ziglang/glibc-abi-tool/tree/main/glibc>.
#[derive(Clone, Copy, Debug)]
pub struct ZigGlibcVersion {
    minor: u8,
}

impl<'de> serde::Deserialize<'de> for ZigGlibcVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;

        let components: Vec<_> = value
            .trim()
            .split('.')
            .map(|part| {
                part.parse::<u8>()
                    .map_err(|_| de::Error::invalid_type(de::Unexpected::Str(part), &"u8"))
            })
            .collect::<Result<_, _>>()?;

        match &components[..] {
            [2, minor] if (16..=38).contains(minor) => Ok(ZigGlibcVersion { minor: *minor }),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &"a valid Zig GLIBC version (2.16 - 2.38)",
            )),
        }
    }
}

impl std::fmt::Display for ZigGlibcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "2.{}", self.minor)
    }
}

impl From<ZigGlibcVersion> for Version {
    fn from(version: ZigGlibcVersion) -> Self {
        Version::new(2, u64::from(version.minor), 0)
    }
}

impl PartialOrd<Version> for ZigGlibcVersion {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        Some(Version::from(*self).cmp(other))
    }
}

impl PartialEq<Version> for ZigGlibcVersion {
    fn eq(&self, other: &Version) -> bool {
        Version::from(*self).eq(other)
    }
}

/// On a GNU host, cross-compile the native addon to only target the oldest supported GLIBC version by VS Code.
///
/// By default, compiling on the host targets the host's GLIBC version, which is usually newer.
/// To prevent that, we need to explicitly cross-compile for the desired GLIBC version.
///
/// This is necessary to retain extension compatibility with as many systems as possible:
/// <https://code.visualstudio.com/docs/supporting/requirements#_additional-linux-requirements>.
pub fn ensure_correct_glibc_for_vscode(
    resolver: NapiResolver,
    output_dir: &Path,
    target: &BuildTarget,
) -> Result<()> {
    // This is only ever required when host-compiling on a GNU system.
    if cfg!(not(target_env = "gnu")) {
        return Ok(());
    }

    let target_triple = match target {
        BuildTarget::ReleaseTarget(target) if target.ends_with("-linux-gnu") => target,
        _ => return Ok(()),
    };

    let target_glibc = NapiConfig::target_glibc(resolver)?;

    let output_artifact = match target_triple.split('-').next() {
        Some("x86_64") => "index.linux-x64-gnu.node",
        Some("aarch64") => "index.linux-arm64-gnu.node",
        _ => bail!("Unsupported target {target_triple} for `cargo-zigbuild`."),
    };
    let output_artifact_path = output_dir.join(output_artifact);

    let is_host_compiling = target_triple.starts_with(std::env::consts::ARCH);
    if is_host_compiling {
        let rust_crate_name = resolver.rust_crate_name();

        // Don't clobber the existing output directory.
        let zigbuild_output = tempfile::tempdir()?;

        // Until `@napi-rs/cli` v3 is released with a fixed `zig` support and a new `--cross-compile`,
        // we explicitly compile ourselves again with `cargo-zigbuild` to target the desired GLIBC
        // version, without having to separately compile on the target platform (e.g. via Docker).
        Command::new("cargo")
            .arg("zigbuild")
            .property("-p", rust_crate_name)
            .flag("--release")
            .property("--target", format!("{target_triple}.{target_glibc}"))
            .property("--target-dir", zigbuild_output.path().to_string_lossy())
            .run();

        // Overwrite the existing artifact with the cross-compiled one.
        let zigbuild_output = zigbuild_output.into_path();
        let artifact_path = zigbuild_output
            .join(target_triple)
            .join("release")
            .join(format!("lib{rust_crate_name}.so"));

        std::fs::copy(artifact_path, &output_artifact_path)?;
    } else {
        // Already cross-compiled with the correct GLIBC version. Just verify for sanity.
    }

    // Verify that the artifact is compatible with the desired GLIBC version.
    let library_glibc_version =
        fetch_min_supported_glibc_version(&output_artifact_path.to_string_lossy())?;

    if target_glibc < library_glibc_version {
        bail!("The compiled artifact {output_artifact_path:?} targets GLIBC {library_glibc_version}, which is higher than the minimum specified version {target_glibc}.");
    }

    Ok(())
}

fn fetch_min_supported_glibc_version(lib_path: &str) -> Result<Version> {
    assert!(
        cfg!(target_env = "gnu"),
        "This is only supported and expected to only ever run on a host GNU system."
    );

    // # Note: `ldd` does not work reliably when inspecting cross-compiled ARM binaries on x86_64
    let output = Command::new("objdump")
        .flag("-T")
        .arg(lib_path)
        .evaluate()?;

    Ok(extract_min_supported_glibc_from_objdump(&output))
}

fn extract_min_supported_glibc_from_objdump(objdump_output: &str) -> Version {
    // Find and capture "2.3.5" (3rd component optional) from "(GLIBC_2.3.5)" substrings
    let regexp =
        regex::Regex::new(r"\(GLIBC_(?<major>[0-9])+\.(?<minor>[0-9]+)(.(?<patch>[0-9]+))?.*\)")
            .unwrap();
    regexp
        .captures_iter(objdump_output)
        .map(|capture| {
            let major = capture.name("major").unwrap().as_str();
            let minor = capture.name("minor").unwrap().as_str();
            let patch = capture.name("patch").map_or("0", |x| x.as_str());
            let [major, minor, patch] = [major, minor, patch].map(|x| x.parse::<u64>().unwrap());

            semver::Version::new(major, minor, patch)
        })
        .max()
        .expect("at least one version to be matched")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_min_supported_glibc_from_objdump() {
        const OUTPUT: &str = r"
        crates/solidity/outputs/npm/package/target/napi/x86_64-unknown-linux-gnu/index.linux-x64-gnu.node:     file format elf64-x86-64

        DYNAMIC SYMBOL TABLE:
        0000000000000000  w   D  *UND*  0000000000000000  Base        __gmon_start__
        0000000000000000      DF *UND*  0000000000000000 (GLIBC_2.3)  __tls_get_addr
        0000000000000000      DF *UND*  0000000000000000 (GLIBC_2.2.5) free
        0000000000000000      D  *UND*  0000000000000000  Base        napi_delete_reference
        0000000000000000      DF *UND*  0000000000000000 (GLIBC_2.14) memcpy
        0000000000000000      DF *UND*  0000000000000000 (GLIBC_2.2.5) malloc
        ";

        let result = extract_min_supported_glibc_from_objdump(OUTPUT);
        assert_eq!(result, Version::new(2, 14, 0));
    }

    #[test]
    fn test_deserialize_zig_glibc_version() -> Result<()> {
        let version = serde_json::from_str::<ZigGlibcVersion>(r#""2.16""#)?;
        assert_eq!(version.minor, 16);

        let version = serde_json::from_str::<ZigGlibcVersion>(r#""2.38""#)?;
        assert_eq!(version.minor, 38);

        let version = serde_json::from_str::<ZigGlibcVersion>(r#""2.39""#);
        assert!(version.is_err());

        let version = serde_json::from_str::<ZigGlibcVersion>(r#""3.20""#);
        assert!(version.is_err());

        // Zig only supports versions without the patch component.
        let version = serde_json::from_str::<ZigGlibcVersion>(r#""2.2.5""#);
        assert!(version.is_err());

        Ok(())
    }

    #[test]
    fn test_display() {
        let version = ZigGlibcVersion { minor: 16 };
        assert_eq!(format!("{version}"), "2.16");
    }
}
