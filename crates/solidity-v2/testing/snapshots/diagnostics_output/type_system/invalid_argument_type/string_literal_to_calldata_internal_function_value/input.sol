// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(function(bytes calldata) internal fp) internal {
        // Expected to flip to failure once #2369 is merged.
        fp("x");
    }
}
