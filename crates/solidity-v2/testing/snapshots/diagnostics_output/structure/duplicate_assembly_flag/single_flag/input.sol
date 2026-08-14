// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Listing the flag once is allowed.
    function once() public pure {
        assembly ("memory-safe") {}
    }

    // An encoded flag listed once is still a single flag.
    function onceEncoded() public pure {
        assembly ("\x6demory-safe") {}
    }

    // An assembly statement need not list any flags.
    function none() public pure {
        assembly {}
    }
}
