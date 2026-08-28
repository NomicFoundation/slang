// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing(uint256[] calldata data) external pure {
        // A calldata slice is not a storage array, so it has no `pop`.
        data[1:2].pop();
    }
}
