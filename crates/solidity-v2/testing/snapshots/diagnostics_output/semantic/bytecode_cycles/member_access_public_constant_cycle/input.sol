// SPDX-License-Identifier: MIT
pragma solidity *;

// Reading the public constant through the library name compiles its value
// into f, the same as a constant without a getter, so A's deployed code
// embeds A's own creation code.

library B {
    bytes public constant CODE = type(A).creationCode;
}

contract A {
    function f() public pure returns (bytes memory) {
        return B.CODE;
    }
}
