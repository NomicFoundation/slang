// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external view returns (uint256) {
        return 0;
    }
}

contract B is A {
    // `A.f` is not `virtual`, so the getter cannot override it.
    uint256 public override f;
}
