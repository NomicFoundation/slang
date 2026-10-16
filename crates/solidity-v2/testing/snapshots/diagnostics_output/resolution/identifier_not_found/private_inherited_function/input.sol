// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() private pure {}
}

contract B is A {
    function g() public pure {
        f();
    }
}
