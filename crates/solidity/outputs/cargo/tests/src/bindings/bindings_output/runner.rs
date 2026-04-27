use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let mut fs = CodegenFileSystem::default();

    super::v1::runner::run(&mut fs, group_name, test_name)?;
    super::v2::runner::run(&mut fs, group_name, test_name)?;

    Ok(())
}
