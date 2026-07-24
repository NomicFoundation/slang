// SPDX-License-Identifier: MIT
pragma solidity *;

// f compiles the library constant's value in, so A's deployed code embeds
// A's own creation code.

library B {
    bytes constant CODE = type(A).creationCode;
}

contract A {
    function f() public pure returns (bytes memory) {
        return B.CODE;
    }
}
