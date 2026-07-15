// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256 self) pure returns (uint256) {
    return self;
}

function f(uint256 self, uint256 other) pure returns (uint256) {
    return self + other;
}

contract C {
    using {f} for uint256;
}
