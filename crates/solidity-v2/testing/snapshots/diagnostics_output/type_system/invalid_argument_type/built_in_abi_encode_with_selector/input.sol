// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public pure {
        abi.encodeWithSelector(1, 2);
    }
}
