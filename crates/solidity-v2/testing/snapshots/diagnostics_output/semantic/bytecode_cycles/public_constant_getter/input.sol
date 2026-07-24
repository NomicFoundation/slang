// SPDX-License-Identifier: MIT
pragma solidity *;

// The getter of a public constant returns the embedded creation code, so A's
// deployed code embeds B and B's creation code embeds A.

contract A {
    bytes public constant CODE = type(B).creationCode;
}

contract B {
    constructor() {
        new A();
    }
}
