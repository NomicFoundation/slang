// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    bytes data;

    function g() public {
        data.push(1000);
    }
}
