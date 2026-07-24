// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract creating itself embeds its own creation bytecode.

contract C {
    constructor() {
        new C();
    }
}
