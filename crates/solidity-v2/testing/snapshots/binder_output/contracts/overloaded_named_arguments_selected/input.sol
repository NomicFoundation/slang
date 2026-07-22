// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint x) external {}
    function f(bool b) external {}
}

contract B {
    function g() public {
        // Not callable via the contract type name, but both the reference and
        // the argument name still resolve against the selected overload.
        A.f({b: true});
    }
}
