// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    int256[10] x;

    function f() public view {
        int256[] storage y = int256[](x);
        y;
    }
}
