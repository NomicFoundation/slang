// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() external {}

    function f() public view returns (address, address) {
        return (address(this.g), this.g.address);
    }
}
