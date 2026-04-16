use crate::define_fixture;

define_fixture!(
    Overrides,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Base
{
    function in_base() internal pure {}
    function override_me() virtual external view {}
}

contract Middle is Base {
    function in_middle() external pure {}
    function override_me() virtual override public view {}
}

contract Inherited is Middle
{
    function override_me() override public pure {}
}
"#,
);
