use std::path::PathBuf;

use infra_utils::cargo::CargoWorkspace;
use strum_macros::EnumIter;

use crate::toolchains::napi::cli::BuildTarget;

pub enum NapiPackageKind {
    Main,
    Platform(String),
}

#[derive(Clone, Copy, EnumIter)]
pub enum NapiResolver {
    Codegen,
    Solidity,
    Testlang,
}

impl NapiResolver {
    pub fn rust_crate_name(self) -> &'static str {
        match self {
            Self::Codegen => "codegen_runtime_node_addon",
            Self::Solidity => "slang_solidity_node_addon",
            Self::Testlang => "slang_testlang_node_addon",
        }
    }

    pub fn main_package_name(self) -> &'static str {
        match self {
            Self::Codegen => "codegen_runtime_npm",
            Self::Solidity => "solidity_npm_package",
            Self::Testlang => "testlang_npm_package",
        }
    }

    pub fn rust_crate_dir(self) -> PathBuf {
        CargoWorkspace::locate_source_crate(self.rust_crate_name()).unwrap()
    }

    pub fn main_package_dir(self) -> PathBuf {
        CargoWorkspace::locate_source_crate(self.main_package_name()).unwrap()
    }

    pub fn platforms_dir(self) -> PathBuf {
        self.main_package_dir().join("platforms")
    }

    pub fn bindings_dir(self) -> PathBuf {
        self.main_package_dir().join(match self {
            // Source templates:
            Self::Codegen => "src/runtime/napi-bindings/generated",
            // Generated Languages:
            Self::Solidity | Self::Testlang => "src/generated/napi-bindings/generated",
        })
    }

    pub fn bindings_output_dir(self) -> PathBuf {
        self.npm_output_dir(&NapiPackageKind::Main)
            .join("napi-bindings/generated")
    }

    pub fn napi_output_dir(self, target: &BuildTarget) -> PathBuf {
        self.main_package_dir()
            .join("target/napi")
            .join(match target {
                BuildTarget::Debug => "debug",
                BuildTarget::ReleaseTarget(target) => target,
            })
    }

    pub fn npm_output_dir(self, kind: &NapiPackageKind) -> PathBuf {
        self.main_package_dir().join("target/npm").join(match kind {
            NapiPackageKind::Main => {
                // __SLANG_NPM_PACKAGE_MAIN_OUTPUT_DIR__ (keep in sync)
                "main"
            }
            NapiPackageKind::Platform(platform) => platform,
        })
    }
}
