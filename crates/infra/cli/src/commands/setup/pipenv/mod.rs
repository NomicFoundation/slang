use anyhow::Result;

use crate::toolchains::pipenv::PipEnv;

pub fn setup_pipenv() -> Result<()> {
    PipEnv::install_packages()
}
