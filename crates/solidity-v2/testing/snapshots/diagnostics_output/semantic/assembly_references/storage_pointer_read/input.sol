// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint[] x;

    function f() public {
        uint[] storage y = x;
        assembly {
            pop(y)
        }
    }
}
