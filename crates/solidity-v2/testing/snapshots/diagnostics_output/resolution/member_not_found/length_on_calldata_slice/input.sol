// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing(uint256[] calldata data) external pure returns (uint256) {
        // A calldata slice carries no members at all, not even `length`.
        return data[1:2].length;
    }
}
