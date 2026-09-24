// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (address, address, address) {
        return (address(-1), address(0), address(0x1234));
    }
}
