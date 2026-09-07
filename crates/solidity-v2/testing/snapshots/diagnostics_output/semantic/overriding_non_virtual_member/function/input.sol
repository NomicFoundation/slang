// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function test() external returns (uint256) {}
}

abstract contract B is A {
    // `A.test` is not `virtual`, so it cannot be overridden.
    function test() external override returns (uint256) {}
}
