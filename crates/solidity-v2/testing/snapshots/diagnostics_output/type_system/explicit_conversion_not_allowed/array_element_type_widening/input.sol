// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8[] memory a) public pure returns (uint256[] memory, uint8[] memory) {
        // Elements are not converted, so their types must match exactly.
        return (uint256[](a), uint8[](a));
    }
}
