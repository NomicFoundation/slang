// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        new X[](1);
    }
}
