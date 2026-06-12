// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 constant a = b * c;
    uint256 constant b = 7;
    uint256 constant c = b + uint256(keccak256(abi.encodePacked(d)));
    uint256 constant d = 2 + a;
}
