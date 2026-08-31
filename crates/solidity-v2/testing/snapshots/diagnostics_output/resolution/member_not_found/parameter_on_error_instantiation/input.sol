// SPDX-License-Identifier: MIT
pragma solidity *;

error E(uint256 amount);

contract C {
    function missing() internal pure returns (uint256) {
        // An error value does not expose the arguments it was built with.
        return E(1).amount;
    }
}
