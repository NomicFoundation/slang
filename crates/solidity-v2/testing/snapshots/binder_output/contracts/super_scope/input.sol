// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract A {
    function foo() public pure virtual returns (string memory) {
        return "A";
    }
}

contract B is A {
    //    A super;
    function foo() public pure virtual override(A) returns (string memory) {
        return super.foo();
    }
}
