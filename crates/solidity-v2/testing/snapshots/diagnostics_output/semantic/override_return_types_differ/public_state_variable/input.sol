// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() external view virtual returns (uint8);
}

contract B is A {
    // The getter returns `uint256`, the overridden function `uint8`.
    uint256 public override f;
}
