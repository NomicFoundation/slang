use std::path::PathBuf;

use infra_utils::cargo::CargoWorkspace;

use crate::toolchains::napi::cli::BuildTarget;

pub enum NapiPackageKind {
    Main,
    Platform(String),
}

pub struct NapiResolver {
    rust_crate: &'static str,
    npm_package: &'static str,
}

impl NapiResolver {
    pub fn testlang() -> Self {
        Self {
            rust_crate: "slang_testlang_node_addon",
            npm_package: "testlang_npm_package",
        }
    }

    pub fn solidity() -> Self {
        Self {
            rust_crate: "slang_solidity_node_addon",
            npm_package: "solidity_npm_package",
        }
    }

    pub fn crate_dir(&self) -> PathBuf {
        CargoWorkspace::locate_source_crate(self.rust_crate).unwrap()
    }

    pub fn main_package_dir(&self) -> PathBuf {
        CargoWorkspace::locate_source_crate(self.npm_package).unwrap()
    }

    pub fn templates_dir(&self) -> PathBuf {
        self.main_package_dir().join("templates")
    }

    pub fn generated_dir(&self) -> PathBuf {
        self.main_package_dir().join("src/generated")
    }

    pub fn platforms_dir(&self) -> PathBuf {
        self.main_package_dir().join("platforms")
    }

    pub fn napi_output_dir(&self, target: &BuildTarget) -> PathBuf {
        self.main_package_dir()
            .join("target/napi")
            .join(match target {
                BuildTarget::Debug => "debug",
                BuildTarget::ReleaseTarget(target) => target,
            })
    }

    pub fn npm_output_dir(&self, kind: &NapiPackageKind) -> PathBuf {
        self.main_package_dir().join("target/npm").join(match kind {
            NapiPackageKind::Main => {
                // __SLANG_NPM_PACKAGE_MAIN_OUTPUT_DIR__ (keep in sync)
                "main"
            }
            NapiPackageKind::Platform(platform) => platform,
        })
    }

    pub fn generated_output_dir(&self) -> PathBuf {
        self.npm_output_dir(&NapiPackageKind::Main)
            .join("generated")
    }
}
