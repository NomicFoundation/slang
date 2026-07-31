// SPDX-License-Identifier: MIT
pragma solidity *;

// The receive and fallback functions both reference B. Slang walks them in
// declaration order, so it reports the receive. solc always walks the
// fallback before the receive, so it reports the fallback. The dependency
// is the same either way, only the expression standing for it differs.

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
