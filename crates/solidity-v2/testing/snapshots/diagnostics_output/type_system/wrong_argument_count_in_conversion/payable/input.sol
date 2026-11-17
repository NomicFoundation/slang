// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public pure {
        payable(address(0), address(1));
    }
}
