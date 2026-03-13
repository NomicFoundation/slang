#![allow(unused_crate_dependencies)]
//! This binary migrates the v1 snapshots to the v2 format.
//!
//! TODO(v2): This is temporary while we keep V1 and V2 tests, eventually we should remove this and keep V2 snapshots only.

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;

fn main() -> Result<()> {
    let v1_snapshots = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?;
    let v2_snapshots = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?;

    solidity_v2_testing_utils::migration::migrate_v1_tests_to_v2(
        &v1_snapshots.join("cst_output"),
        &v2_snapshots.join("cst_output"),
    )
}
