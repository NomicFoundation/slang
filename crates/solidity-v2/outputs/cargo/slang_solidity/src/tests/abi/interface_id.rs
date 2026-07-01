use crate::ast::ContractBase;
use crate::define_fixture;

define_fixture!(
    InterfaceIds,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

interface IERC165 {
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

interface IThing is IERC165 {
    function transfer(address to, uint256 amount) external returns (bool);
}

contract Implementer is IThing {}
"#,
);

#[test]
fn interface_id_xors_selectors_including_inherited() {
    let unit = InterfaceIds::build_compilation_unit();
    let implementer = unit
        .find_contract_by_name("Implementer")
        .next()
        .expect("Implementer contract can be found");

    let interface_ids: Vec<u32> = implementer
        .linearised_bases()
        .into_iter()
        .filter_map(|base| match base {
            ContractBase::Interface(interface) => Some(interface.compute_interface_id()),
            ContractBase::Contract(_) => None,
        })
        .collect();

    // `type(IERC165).interfaceId` is just the selector of `supportsInterface(bytes4)`.
    assert!(interface_ids.contains(&0x01ff_c9a7));
    // `type(IThing).interfaceId` folds in every inherited-interface function (here
    // `transfer(address,uint256)` = 0xa9059cbb), not only IThing's own members.
    assert!(interface_ids.contains(&(0x01ff_c9a7 ^ 0xa905_9cbb)));
}
