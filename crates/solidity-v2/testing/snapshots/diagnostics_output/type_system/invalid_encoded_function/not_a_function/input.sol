// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        abi.encodeCall(1, ());
    }
}
