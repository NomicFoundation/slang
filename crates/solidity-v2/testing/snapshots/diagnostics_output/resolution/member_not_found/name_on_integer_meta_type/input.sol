// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing() internal pure returns (string memory) {
        // `name` is a member of contract meta-types only.
        return type(int256).name;
    }

    function present() internal pure returns (int256) {
        return type(int256).min;
    }
}
