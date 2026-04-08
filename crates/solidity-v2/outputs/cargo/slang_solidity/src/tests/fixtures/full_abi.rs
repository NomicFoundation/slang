use crate::define_fixture;

define_fixture!(
    FullAbi,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

interface Base {
    error SomethingWrong(string);
    event BaseEvent(uint a, string m) anonymous;
}

contract Test is Base {
    bytes32 public b;
    constructor() { b = hex"12345678901234567890123456789012"; }
    event Event(uint indexed a, bytes32 b);
    error InsufficientBalance(uint256 available, uint256 required);
    function foo(uint a) public { emit Event(a, b); }
    receive() external payable { }
    fallback() external { }
}
"#,
);
