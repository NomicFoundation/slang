use std::path::PathBuf;

use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use semver::Version;

pub const SOLC_VERSION: Version = Version::new(0, 8, 20);

const SOURCES: &[&str] = &[
    "node_modules/@openzeppelin/contracts/governance/Governor.sol",
    "node_modules/@openzeppelin/contracts/token/ERC20/ERC20.sol",
    "node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol",
    "node_modules/@openzeppelin/contracts/utils/math/SafeCast.sol",
    "node_modules/@openzeppelin/contracts/utils/structs/EnumerableMap.sol",
];

pub struct SourceFile {
    pub path: PathBuf,
    pub contents: String,
}

impl SourceFile {
    pub fn load_all() -> Vec<Self> {
        let crate_dir = CargoWorkspace::locate_source_crate("solidity_testing_perf").unwrap();

        SOURCES
            .iter()
            .map(|relative_path| {
                let path = crate_dir.join(relative_path);
                let contents = path.read_to_string().unwrap();

                Self { path, contents }
            })
            .collect()
    }
}
