// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    error E();

    function f() public pure returns (bytes4, bytes4) {
        // An error is not a value: its selector is.
        return (bytes4(E), E.selector);
    }
}
