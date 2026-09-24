//! Builds a v2 compilation unit from a corpus record.

use semver::Version;
use slang_solidity_v2::compilation::{CompilationUnit, Configuration, FileId, ImportResolver};
use slang_solidity_v2::diagnostics::kinds::compilation::UnresolvedImport;
use slang_solidity_v2::utils::{EvmTarget, LanguageVersion};
use solidity_testing_utils::import_resolver::{
    ImportRemap, ImportResolver as CorpusResolver, SourceMap,
};
use solidity_v2_testing_utils::evm_targets::{default_evm_target, parse_evm_target_name};

use crate::corpus::CorpusContract;

/// Why a record cannot be compiled at all; counted separately from failures.
#[derive(Debug)]
pub enum Skip {
    UnsupportedLanguageVersion(String),
    UnknownEvmTarget(String),
}

impl std::fmt::Display for Skip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedLanguageVersion(version) => {
                write!(f, "unsupported language version {version}")
            }
            Self::UnknownEvmTarget(name) => write!(f, "unknown EVM target {name}"),
        }
    }
}

pub fn language_version(record: &CorpusContract) -> Result<LanguageVersion, Skip> {
    Version::parse(&record.version)
        .ok()
        .and_then(|version| LanguageVersion::try_from(version).ok())
        .ok_or_else(|| Skip::UnsupportedLanguageVersion(record.version.clone()))
}

/// The record's own target, else solc's default for its compiler version.
pub fn evm_target(record: &CorpusContract, version: LanguageVersion) -> Result<EvmTarget, Skip> {
    let name = record.evm_version.as_deref().or_else(|| {
        record
            .settings
            .get("evmVersion")
            .and_then(|value| value.as_str())
    });
    match name {
        Some(name) => {
            parse_evm_target_name(name).ok_or_else(|| Skip::UnknownEvmTarget(name.to_owned()))
        }
        None => Ok(default_evm_target(version)),
    }
}

pub fn create(
    record: &CorpusContract,
    version: LanguageVersion,
    target: EvmTarget,
) -> CompilationUnit {
    let source_maps = record
        .sources
        .keys()
        .map(|path| SourceMap {
            source_id: path.clone(),
            virtual_path: path.clone(),
        })
        .collect();
    let import_remaps = record
        .remappings
        .iter()
        .filter_map(|remapping| {
            // The corpus keeps solc's `[context:]prefix=target` spelling; the resolver
            // wants the context separator present.
            let remapping = if remapping.contains(':') {
                remapping.clone()
            } else {
                format!(":{remapping}")
            };
            ImportRemap::new(&remapping).ok()
        })
        .filter(|remap| !remap.has_known_bug())
        .collect();

    CompilationUnit::create(Configuration {
        language_version: version,
        evm_target: target,
        sources: record
            .sources
            .iter()
            .map(|(path, content)| (FileId::from(path.as_str()), content.as_str())),
        resolver: Resolver(CorpusResolver {
            import_remaps,
            source_maps,
        }),
    })
}

struct Resolver(CorpusResolver);

impl ImportResolver for Resolver {
    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        self.0
            .resolve_import(source_file_id.as_str(), import_path)
            .map(FileId::from)
            .ok_or_else(|| UnresolvedImport {
                reason: format!(
                    "cannot resolve `{import_path}` from `{source}`",
                    source = source_file_id.as_str()
                ),
            })
    }
}
