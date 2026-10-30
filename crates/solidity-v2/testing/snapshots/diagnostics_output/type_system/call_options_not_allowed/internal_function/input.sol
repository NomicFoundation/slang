// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function i() internal {}

    function e() external {}

    function missing() internal {
        // An internal function is not called by a message call.
        i{gas: 1000}();
    }

    function present() internal {
        this.e{gas: 1000}();
    }
}
