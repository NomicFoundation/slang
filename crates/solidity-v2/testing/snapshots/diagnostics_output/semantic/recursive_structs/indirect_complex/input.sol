// SPDX-License-Identifier: MIT
pragma solidity *;

// A longer cycle through a mix of direct members and fixed-size arrays is still
// recursive. The cycle here is A to D to C to B and back to A.

contract Test {
    struct A {
        address addr;
        uint256 count;
        D[1] x;
    }
    struct B {
        A x;
    }
    struct C {
        B[1] x;
    }
    struct D {
        C x;
    }
}
