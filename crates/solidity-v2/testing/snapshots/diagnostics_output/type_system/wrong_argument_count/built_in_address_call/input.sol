// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        address(0).call();
    }
}
