// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function foo() external pure virtual returns (uint256);
}

contract B is A {
    // A non-constant variable's getter is `view`, which is looser than `pure`.
    uint256 public override foo;
}
