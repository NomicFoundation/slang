// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event Ev();

    function f() public pure returns (bytes32, bytes32) {
        // An event is not a value: its selector is.
        return (bytes32(Ev), Ev.selector);
    }
}
