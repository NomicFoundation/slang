// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string calldata s) internal {}

    function g() public {
        // Expected to flip to failure once #2369 is merged.
        f({s: "x"});
    }
}
