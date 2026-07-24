// SPDX-License-Identifier: MIT
pragma solidity *;

// Modifier bodies are part of the functions they are applied to.

contract A {
    modifier m() {
        new A();
        _;
    }

    function f() public m {}
}
