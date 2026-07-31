// SPDX-License-Identifier: MIT
pragma solidity *;

// Private library functions execute within the caller's bytecode just like
// internal ones, so the library reaches its own creation code.

library L {
    function f() public pure returns (bytes memory) {
        return g();
    }

    function g() private pure returns (bytes memory) {
        return type(L).creationCode;
    }
}
