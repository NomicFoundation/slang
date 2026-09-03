// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function foo() external {}

    // A function with call options applied can only be called.
    function pick(bool c) public view returns (function() external) {
        return c ? this.foo : this.foo{gas: 4};
    }
}
