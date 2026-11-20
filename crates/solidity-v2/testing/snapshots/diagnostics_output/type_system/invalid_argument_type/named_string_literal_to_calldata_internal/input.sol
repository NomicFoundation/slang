// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string calldata s) internal {}

    function g() public {
        f({s: "x"});
    }
}
