// SPDX-License-Identifier: MIT
pragma solidity *;

// A modifier invocation names an event.
contract C {
    event E();

    function f() public E {}
}
