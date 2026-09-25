// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public view returns (address payable, address) {
        return (payable(this), address(this));
    }
}
