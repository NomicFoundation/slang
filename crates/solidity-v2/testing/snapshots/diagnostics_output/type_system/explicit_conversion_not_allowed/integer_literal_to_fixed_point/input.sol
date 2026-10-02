// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        // A literal converts when it fits the fixed-point type exactly.
        fixed x = fixed(1);
        ufixed y = ufixed(2);
        fixed w = fixed(1.5);
        ufixed8x1 z = ufixed8x1(300);
        x;
        y;
        w;
        z;
    }
}
