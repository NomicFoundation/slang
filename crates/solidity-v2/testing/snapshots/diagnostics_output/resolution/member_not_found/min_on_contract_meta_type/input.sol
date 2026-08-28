// SPDX-License-Identifier: MIT
pragma solidity *;

contract Other {}

contract C {
    function missing() internal pure returns (uint256) {
        // `min` is a member of integer meta-types only.
        return type(Other).min;
    }

    function present() internal pure returns (string memory) {
        return type(Other).name;
    }
}
