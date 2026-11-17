// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external {}
}

contract C {
    function missing() internal {
        // A library call runs in the caller's context.
        L.f{gas: 1000}();
    }
}
