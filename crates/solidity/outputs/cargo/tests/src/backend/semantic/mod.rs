use anyhow::Result;

use super::sample::build_compilation_unit;

#[test]
fn test_create_semantic_analysis() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    assert_eq!(unit.files().len(), semantic.files().len());

    let main_ir = semantic
        .get_file_ast_root("main.sol")
        .expect("main.sol is a file of the compilation unit");
    let ownable_ir = semantic
        .get_file_ast_root("ownable.sol")
        .expect("ownable.sol is a file in the compilation unit");

    assert_eq!(main_ir.file_id(), "main.sol");
    assert_eq!(ownable_ir.file_id(), "ownable.sol");

    assert_eq!(main_ir.contracts().count(), 1);
    assert_eq!(ownable_ir.contracts().count(), 1);

    let counter_contract = main_ir.contracts().next().unwrap();
    assert_eq!(counter_contract.name().unparse(), "Counter");
    assert_eq!(counter_contract.inheritance_types().count(), 1);

    let counter_base = counter_contract.inheritance_types().next().unwrap();
    let ownable_contract = counter_base
        .type_name()
        .resolve_to_definition()
        .expect("Counter base is resolved")
        .as_contract()
        .expect("Counter base is a contract");

    assert_eq!(ownable_contract.name().unparse(), "Ownable");

    Ok(())
}
