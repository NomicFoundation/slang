// SPDX-License-Identifier: MIT
pragma solidity *;

// The cycle runs through two free function calls.

contract D {
    function f() public {
        l();
    }
}

contract C {
    constructor() {
        new D();
    }
}

function l() {
    s();
}

function s() {
    new C();
}
