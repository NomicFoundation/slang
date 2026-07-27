// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // Invalid: a trailing empty component in a read-position tuple.
        ~(0, );
    }
}
