use std::path::PathBuf;

use infra_utils::cargo::CargoWorkspace;

use crate::toolchains::napi::cli::BuildTarget;

pub enum NapiPackageKind {
    Main,
    Platform(String),
}

pub struct NapiResolver;

impl NapiResolver {
    pub fn crate_dir() -> PathBuf {
        CargoWorkspace::locate_source_crate("slang_solidity_node_addon").unwrap()
    }

    pub fn main_package_dir() -> PathBuf {
        CargoWorkspace::locate_source_crate("solidity_npm_package").unwrap()
    }

    pub fn generated_dir() -> PathBuf {
        Self::main_package_dir().join("src/generated")
    }

    pub fn platforms_dir() -> PathBuf {
        Self::main_package_dir().join("platforms")
    }

    pub fn napi_output_dir(target: &BuildTarget) -> PathBuf {
        Self::main_package_dir()
            .join("target/napi")
            .join(match target {
                BuildTarget::Debug => "debug",
                BuildTarget::ReleaseTarget(target) => target,
            })
    }

    pub fn npm_output_dir(kind: &NapiPackageKind) -> PathBuf {
        Self::main_package_dir()
            .join("target/npm")
            .join(match kind {
                NapiPackageKind::Main => "main",
                NapiPackageKind::Platform(platform) => platform,
            })
    }

    pub fn generated_output_dir() -> PathBuf {
        Self::npm_output_dir(&NapiPackageKind::Main).join("generated")
    }
}
