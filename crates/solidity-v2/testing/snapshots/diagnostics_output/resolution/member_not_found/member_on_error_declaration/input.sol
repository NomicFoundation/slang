// SPDX-License-Identifier: MIT
pragma solidity *;

error E(uint256 amount);

contract C {
    function missing() internal pure {
        // An error declaration only carries `selector`.
        E.amount;
    }
}
