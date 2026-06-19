// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() internal {}
    function f(uint x) external {}
}

contract B is A {
    function call_internal() public {
        A.f(); // an internal function can be called from derived contracts
    }
    function call_external() public {
        A.f(1); // an external function can't be called in this way
    }
}
