// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        abi.encode({a: 1});
    }
}
