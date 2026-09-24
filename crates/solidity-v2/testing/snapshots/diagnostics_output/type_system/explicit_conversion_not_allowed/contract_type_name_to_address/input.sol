// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

contract C {
    function f() public pure returns (address, address) {
        // Only a library name converts to its address.
        return (address(C), address(L));
    }
}
