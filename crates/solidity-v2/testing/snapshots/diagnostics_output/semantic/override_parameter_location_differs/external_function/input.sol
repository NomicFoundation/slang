// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint256[] calldata) external pure virtual {}

    function g(uint256[] calldata) external view virtual {}
}

// Overriding an `external` function may change the data locations of its
// parameters, since the encoding of the call data doesn't change.
contract B is A {
    function f(uint256[] memory) public pure override {}

    function g(uint256[] memory) public view override {}
}
