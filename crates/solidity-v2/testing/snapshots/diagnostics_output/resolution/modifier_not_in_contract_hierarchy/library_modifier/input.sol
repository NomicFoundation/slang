// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract invokes a library's modifier, which only the library itself can
// use.
library L {
    modifier m() {
        _;
    }
}

contract C {
    function f() public L.m {}
}
