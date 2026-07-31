// SPDX-License-Identifier: MIT
pragma solidity *;

// The arguments of a modifier invocation run within the function.

contract A {
    modifier m(uint256 x) {
        _;
    }

    function f() public m(g()) {}

    function g() internal returns (uint256) {
        new B();
        return 0;
    }
}

contract B {
    constructor() {
        new A();
    }
}
