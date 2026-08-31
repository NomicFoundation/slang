// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier tagged(uint256 value) {
        _;
    }

    // Valid: a modifier invocation accepts positional arguments.
    function positional() public tagged(1) {}

    // Named arguments are not allowed in a modifier invocation.
    function named() public tagged({value: 2}) {}
}
