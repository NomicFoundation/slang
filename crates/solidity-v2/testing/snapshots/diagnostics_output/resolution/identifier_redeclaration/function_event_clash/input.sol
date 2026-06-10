// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    event dup();

    function dup() public pure returns (uint) {
        return 1;
    }
}
