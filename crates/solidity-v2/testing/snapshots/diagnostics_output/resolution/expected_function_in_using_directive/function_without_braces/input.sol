// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256 self) pure returns (uint256) {
    return self;
}

using f for uint256;
