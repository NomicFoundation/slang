// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function test() internal virtual returns (uint256);
}

contract B is A {
    // A getter can only override an `external` function.
    uint256 public override test = 42;
}
