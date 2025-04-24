use std::rc::Rc;

use anyhow::{anyhow, Result};
use slang_solidity::backend::l2_flat_contracts::transformer::Transformer;
use slang_solidity::backend::{l1_typed_cst, l2_flat_contracts};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

struct AstToL1 {}

impl Transformer for AstToL1 {
    fn transform_contract_definition(
        &mut self,
        source: &l1_typed_cst::ContractDefinition,
    ) -> l2_flat_contracts::ContractDefinition {
        let node_id = source.node_id;
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
        let name = Rc::clone(&source.name);
        let members = self.transform_contract_members(&source.members);
        let inheritance_types = source
            .specifiers
            .iter()
            .find_map(|specifier| {
                if let l1_typed_cst::ContractSpecifier::InheritanceSpecifier(inheritance) =
                    specifier
                {
                    Some(self.transform_inheritance_types(&inheritance.types))
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let storage_layout = source.specifiers.iter().find_map(|specifier| {
            if let l1_typed_cst::ContractSpecifier::StorageLayoutSpecifier(storage_layout) =
                specifier
            {
                Some(self.transform_storage_layout_specifier(storage_layout))
            } else {
                None
            }
        });

        Rc::new(l2_flat_contracts::ContractDefinitionStruct {
            node_id,
            abstract_keyword,
            name,
            members,
            inheritance_types,
            storage_layout,
        })
    }
}

#[test]
fn test_build_l1_from_ast() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###,
    );
    assert!(output.is_valid());

    let ast_source = l1_typed_cst::builder::build_source_unit(output.create_tree_cursor())
        .map_err(|s| anyhow!(s))?;

    let mut transformer = AstToL1 {};
    let l1 = transformer.transform_source_unit(&ast_source);

    assert_eq!(2, l1.members.len());

    let l2_flat_contracts::SourceUnitMember::ContractDefinition(base_contract) = &l1.members[0]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let l2_flat_contracts::SourceUnitMember::ContractDefinition(test_contract) = &l1.members[1]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Test", test_contract.name.unparse());
    assert_eq!(1, test_contract.inheritance_types.len());
    assert_eq!(
        "Base",
        test_contract.inheritance_types[0]
            .type_name
            .iter()
            .map(|node| node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    );
    assert!(test_contract.storage_layout.is_some());

    Ok(())
}
