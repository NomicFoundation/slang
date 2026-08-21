// SPDX-License-Identifier: MIT
pragma solidity *;

// Distinguishable free overloads.
function ok(uint256 a) pure returns (uint256) {
    return a;
}

function ok(bytes memory a) pure returns (uint256) {
    return a.length;
}

// Two free functions a call could not tell apart.
function duplicated(uint256 a) pure returns (uint256) {
    return a;
}

function duplicated(uint256 a) view returns (uint256) {
    return a + block.number;
}
