// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 public x;
}

contract B is A {
    // A public state variable cannot be overridden, not even by a function
    // with its getter's signature.
    function x() public view override returns (uint256) {
        return 0;
    }
}
