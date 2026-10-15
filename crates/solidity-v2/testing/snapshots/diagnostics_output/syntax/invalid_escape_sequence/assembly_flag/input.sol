// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    function f() public pure {
        // Assembly flags are Yul string literals too.
        assembly ("\q") {}
    }
}
