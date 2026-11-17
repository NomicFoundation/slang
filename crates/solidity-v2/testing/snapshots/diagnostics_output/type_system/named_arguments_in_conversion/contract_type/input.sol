// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public pure {
        C({x: address(0)});
    }
}
