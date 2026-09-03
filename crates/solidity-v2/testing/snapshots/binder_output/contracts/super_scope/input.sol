// SPDX-License-Identifier: MIT
pragma solidity *;

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
