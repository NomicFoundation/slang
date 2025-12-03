use slang_solidity::backend::{build_binder_output, BinderOutput};

pub fn setup(project: &str) -> BinderOutput {
    build_binder_output(
        super::setup::setup(project)
            .build_compilation_unit()
            .unwrap(),
    )
}
