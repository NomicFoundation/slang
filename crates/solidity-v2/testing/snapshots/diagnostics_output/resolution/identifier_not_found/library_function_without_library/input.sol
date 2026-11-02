// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f() external pure {}
}

contract C {
    function g() public pure {
        f();
    }
}
