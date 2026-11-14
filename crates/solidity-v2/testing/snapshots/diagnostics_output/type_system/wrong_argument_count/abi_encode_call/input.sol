// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        abi.encodeCall(this.g);
    }
}
