// SPDX-License-Identifier: MIT
pragma solidity *;

// Mutual runtime code access is a bytecode cycle just like `new`.

contract A {
    function f() public pure returns (bytes memory) {
        return type(B).runtimeCode;
    }
}

contract B {
    function f() public pure returns (bytes memory) {
        return type(A).runtimeCode;
    }
}
