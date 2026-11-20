// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint a) internal pure {
        // Reported once, at the call options, and not again at the call.
        a{gas: 1000}();
    }
}
