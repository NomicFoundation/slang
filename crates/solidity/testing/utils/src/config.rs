use std::path::{Path, PathBuf};

use infra_utils::paths::PathExtensions;

pub struct Project {
    pub hash: &'static str,
    pub name: &'static str,
}

pub struct File {
    pub hash: &'static str,
    pub file: &'static str,
    pub name: &'static str,
}

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
pub const PROJECTS: &[Project] = &[
    // Large structure of flat imports
    Project {
        hash: "0x01a11a5A999E57E1B177AA2fF7fEA957605adA2b",
        name: "mooniswap",
    },
    // Large structure of imports, including a circular path
    Project {
        hash: "0x01abc00E86C7e258823b9a055Fd62cA6CF61a163",
        name: "weighted_pool",
    },
    // Used protocol
    Project {
        hash: "0x000000000004444c5dc75cB358380D2e3dE08A90",
        name: "uniswap",
    },
    // Used protocol
    Project {
        hash: "0xcA11bde05977b3631167028862bE2a173976CA11",
        name: "multicall3",
    },
    // Used protocol
    Project {
        hash: "0xba5Ed099633D3B313e4D5F7bdc1305d3c28ba5Ed",
        name: "create_x",
    },
    // Used protocol
    Project {
        hash: "0x3F78BBD206e4D3c504Eb854232EdA7e47E9Fd8FC",
        name: "ui_pool_data_provider_v3",
    },
    // Deep CST structure
    Project {
        hash: "0x56681458E00CafE1206313D2D033946f458FDEfD",
        name: "cooldogs",
    },
];

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
pub const FILES: &[File] = &[
    // Largest file with trivia in mainnet shard 00
    File {
        hash: "0x00aB290CC289F818a9E80eBaF18685E353DF16F0",
        file: "contracts/OneStepLeverage.f.sol",
        name: "one_step_leverage_f",
    },
    // Largest file in mainnet shard 00
    File {
        hash: "0x0000000000d6A44FaCfBe05Faaee691aFaCC0f81",
        file: "lib/seaport-types/src/helpers/PointerLibraries.sol",
        name: "pointer_libraries",
    },
    // File in the 75% of LOCs in mainnet shard 01
    File {
        hash: "0x01665987bC6725070e56d160d75AA19d8B73273e",
        file: "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol",
        name: "merkle_proof",
    },
];

pub fn working_dir_path() -> PathBuf {
    Path::repo_path("target/benchmarks-inputs")
}
