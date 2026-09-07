// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() external virtual returns (uint256);
}

contract B is A {
    // The getter overrides `A.f` without the `override` specifier.
    uint256 public f;
}
