// SPDX-License-Identifier: MIT
pragma solidity *;

error E(uint256 amount);

contract C {
    function missing() internal pure returns (bytes4) {
        // The named-argument form builds the same error value as the positional one.
        return E({amount: 1}).selector;
    }
}
