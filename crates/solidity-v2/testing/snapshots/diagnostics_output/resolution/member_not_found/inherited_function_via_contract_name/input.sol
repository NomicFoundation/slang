// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
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
}
