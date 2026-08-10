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
    function balanceOf(address owner) external view returns (uint256);
}

abstract contract Implementer is IThing {}

interface IMarker is IERC165 {}

abstract contract MarkerImplementer is IMarker {}
"#,
);

#[test]
fn interface_id_xors_own_selectors_excluding_inherited() {
    let unit = InterfaceIds::build_compilation_unit();
    let implementer = unit
        .find_contract_by_name("Implementer")
        .next()
        .expect("Implementer contract can be found");

    let bases = implementer.linearised_bases();
    assert_eq!(bases.len(), 3);

    let ContractBase::Interface(thing) = &bases[1] else {
        panic!("IThing base is not an interface");
    };
    // transfer(address,uint256) ^ balanceOf(address)
    assert_eq!(
        thing.compute_interface_id(),
        Some(0xa905_9cbb ^ 0x70a0_8231)
    );

    let ContractBase::Interface(erc165) = &bases[2] else {
        panic!("IERC165 base is not an interface");
    };
    assert_eq!(erc165.compute_interface_id(), Some(0x01ff_c9a7)); // supportsInterface(bytes4)
}

#[test]
fn interface_id_is_zero_without_own_functions() {
    let unit = InterfaceIds::build_compilation_unit();
    let implementer = unit
        .find_contract_by_name("MarkerImplementer")
        .next()
        .expect("MarkerImplementer contract can be found");

    let bases = implementer.linearised_bases();
    let ContractBase::Interface(marker) = &bases[1] else {
        panic!("IMarker base is not an interface");
    };
    assert_eq!(marker.compute_interface_id(), Some(0));
}
