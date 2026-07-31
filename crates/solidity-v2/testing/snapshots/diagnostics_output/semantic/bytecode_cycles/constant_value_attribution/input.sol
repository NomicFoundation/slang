// SPDX-License-Identifier: MIT
pragma solidity *;

// Base reaches X only through the constant. Derived reaches it through the
// constant and through its own expression. A unit's own references are
// recorded before the constants it uses, so slang reports Derived's own
// expression and Base's constant separately. solc compiles a constant in
// where it is used, reports the constant's expression for both, and emits
// one error less. The dependency itself is the same either way, only the
// expression standing for it differs, so slang keeps the cheaper order.

bytes constant K = type(X).creationCode;

contract Base {
    function f() public pure returns (bytes memory) {
        return K;
    }
}

contract Derived is Base {
    function g() public pure returns (bytes memory) {
        bytes memory a = K;
        return abi.encodePacked(a, type(X).creationCode);
    }
}

contract X {
    constructor() {
        new Base();
    }
}
