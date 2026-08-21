// SPDX-License-Identifier: MIT
pragma solidity *;

// Multiple identical declarations produce diagnostics on each duplicate.
function repeated() pure returns (uint256) {
    return 1;
}

function repeated() pure returns (uint256) {
    return 2;
}

function repeated() pure returns (uint256) {
    return 3;
}

contract C {
    // Same for contract members where memory and calldata locations are
    // indistinguishable.
    function mixedLocations(uint256[] memory a) public pure returns (uint256) {
        return a.length;
    }

    function mixedLocations(uint256[] memory a) public pure returns (uint256) {
        return a.length + 1;
    }

    function mixedLocations(uint256[] calldata a) external pure returns (uint256) {
        return a.length + 2;
    }
}
