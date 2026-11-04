// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        abi.encodeWithSelector({selector: bytes4(0)});
    }
}
