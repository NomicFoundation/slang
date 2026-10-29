// SPDX-License-Identifier: MIT
pragma solidity *;

function ff() {}

contract C {
    function missing() internal pure returns (bytes4) {
        // A free function is only ever called internally.
        return ff.selector;
    }
}
