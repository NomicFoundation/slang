// SPDX-License-Identifier: MIT
pragma solidity *;

// The receive and fallback functions both reference B. Slang and solc both
// walk the fallback before the receive, whatever the declaration order, so
// they report the fallback. The dependency is the same either way, only the
// expression standing for it differs.

contract A {
    receive() external payable {
        new B();
    }

    fallback() external {
        type(B).creationCode;
    }
}

contract B {
    constructor() {
        new A();
    }
}
