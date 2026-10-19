// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    uint256 internal x;
}

function f() view returns (uint256) {
    return D.x;
}
