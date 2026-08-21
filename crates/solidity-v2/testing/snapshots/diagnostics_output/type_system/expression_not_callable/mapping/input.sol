// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/mapping_not_callable.sol
// Calling a mapping, which is indexed and not called.

contract C {
    mapping(uint256 => uint256) m;

    // TypeError 5704: This expression is not callable.
    uint256 a = m(1000);

    // Indexing the same mapping is fine.
    uint256 b = m[1000];
}
