// SPDX-License-Identifier: MIT
pragma solidity *;

// The first item is a plain function value; only the second, which has call
// options applied, has no mobile type.

contract C {
    function foo() external {}

    function f() public view {
        [this.foo, this.foo{gas: 4}];
    }
}
