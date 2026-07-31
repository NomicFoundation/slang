// SPDX-License-Identifier: MIT
pragma solidity *;

// Base and Derived both reach X, and Derived inherits the fallback. Slang
// walks entry points in linearised function list order, which puts the
// unnamed fallback first for both contracts, so both are attributed to the
// same `new X()` and the second report is dropped as already reported. solc
// walks the external interface functions before the fallback, attributes
// Base to `f` and Derived to `g`, and reports both. The dependencies
// themselves match, only the expressions standing for them differ.

contract Base {
    fallback() external {
        new X();
    }

    function f() public pure returns (bytes memory) {
        return type(X).creationCode;
    }
}

contract Derived is Base {
    function g() public {
        new X();
    }
}

contract X {
    constructor() {
        new Base();
    }
}
