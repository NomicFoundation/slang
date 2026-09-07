// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function test() public virtual returns (uint256);
}

contract B is A {
    // A getter can only override an `external` function, not a `public` one.
    uint256 public override test = 42;
}
