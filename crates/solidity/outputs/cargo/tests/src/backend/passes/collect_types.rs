use anyhow::Result;
use slang_solidity::backend::{ast, passes::collect_types};

#[test]
fn test_collect_types_pass() -> Result<()> {
    let unit = super::build_compilation_unit()?;
    let ast = ast::CompilationUnit::build(&unit).unwrap();

    let mut pass = collect_types::Pass::new();
    for ast_unit in ast.files.values() {
        ast::visitor::accept_source_unit(ast_unit, &mut pass);
    }

    assert_eq!(2, pass.types.len());
    assert!(pass
        .types
        .iter()
        .find(|r#type| matches!(r#type, collect_types::Type::Contract(name) if name == "Ownable"))
        .is_some());
    assert!(pass
        .types
        .iter()
        .find(|r#type| matches!(r#type, collect_types::Type::Contract(name) if name == "Counter"))
        .is_some());

    Ok(())
}
