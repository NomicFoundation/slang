// SPDX-License-Identifier: MIT
pragma solidity *;

contract Strings {
    function f() public pure {
        // Assembly flags are Yul string literals too.
        assembly ("café") {}
    }
}
