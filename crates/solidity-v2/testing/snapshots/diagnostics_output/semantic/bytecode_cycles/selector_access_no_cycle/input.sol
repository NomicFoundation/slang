// SPDX-License-Identifier: MIT
pragma solidity *;

// `C.f` here is only a declaration used for its selector, not a call, so A
// does not embed C's bytecode and there is no cycle.

contract A {
    function sel() public pure returns (bytes4) {
        return C.f.selector;
    }
}

contract C {
    function f() public {
        new A();
    }
}
