// SPDX-License-Identifier: MIT
pragma solidity *;

// The named function and the fallback both reference B. Slang walks the
// linearised function list, which puts the unnamed fallback first, so it
// reports the fallback. solc walks the external interface first and only
// then the fallback, so it reports `f`. The dependency is the same either
// way, only the expression standing for it differs.

contract A {
    fallback() external {
        new B();
    }

    function f() public pure returns (bytes memory) {
        return type(B).creationCode;
    }
}

contract B {
    constructor() {
        new A();
    }
}
