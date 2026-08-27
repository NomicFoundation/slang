mod eip712;
mod entries;
mod interface_id;
mod internal_signature;
mod selectors;
mod storage_layout;
mod type_conversion;

use super::fixtures;
use crate::define_fixture;

define_fixture!(
    AbiWithTuples,
    file: "main.sol", r#"
pragma solidity *;
contract Test {
    struct S { uint a; uint[] b; T[] c; }
    struct T { uint x; uint y; }

    function f(S memory, T memory, uint x) public pure {}
    function g() public pure returns (S memory s, T memory t, uint) {}

    T public t;  // getter returns (uint x, uint y)
    function t_components() public view returns (uint, uint) {
        return (t.x, t.y);
    }
    function t_struct() public view returns (T memory) {
        return t;
    }
}
"#,
);

define_fixture!(
    FullAbi,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

interface ITest {
    error SomethingWrong(string);
    event BaseEvent(uint a, string m) anonymous;
}

contract Base {
    uint[] public xs;
    bool a;
}

contract Test is ITest, Base {
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

define_fixture!(
    LibraryAbi,
    file: "main.sol", r#"
pragma solidity *;
library L {
    struct S { mapping(uint => uint) m; }
    type U is uint64;

    function f(S storage s, uint x) external returns (uint) {}
    function g(uint[] storage xs) public {}
    function h(uint x) external pure returns (uint) {}
    function i(uint x) internal pure returns (uint) {}
    function j(U u) external pure {}
}
"#,
);

define_fixture!(
    LibraryUdvtAbi,
    file: "main.sol", r#"
pragma solidity *;
library L {
    type U is uint64;

    function k(U[] memory xs) external pure {}
    function q(U[2] memory xs) external pure {}
    function m(mapping(uint256 => U) storage x) external {}
}
"#,
);
