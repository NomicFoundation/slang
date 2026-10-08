// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    struct S {
        uint256 x;
    }

    function f() internal pure returns (uint256) {
        return 1;
    }
}

contract B is A {}

contract C is B {
    function inherited() internal pure returns (uint256) {
        return B.f();
    }

    function declared() internal pure returns (uint256) {
        return A.f();
    }

    // A type name still reaches inherited declarations.
    function typeName() internal pure {
        B.S memory s;
        s;
    }
}
