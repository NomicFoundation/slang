// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[] memory a) public pure returns (uint8[] memory, uint256[] memory) {
        return (uint8[](a), uint256[](a));
    }
}
