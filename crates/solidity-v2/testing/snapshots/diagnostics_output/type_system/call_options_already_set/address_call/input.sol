// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        address(this).call{gas: 1}{value: 1}("");
    }
}
