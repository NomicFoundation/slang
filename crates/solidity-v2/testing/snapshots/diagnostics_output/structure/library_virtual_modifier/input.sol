// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    // A modifier in a library cannot be marked `virtual`.
    modifier m1() virtual {
        _;
    }

    // A non-virtual modifier in a library is allowed.
    modifier m2() {
        _;
    }
}

contract C {
    // A `virtual` modifier is allowed outside a library.
    modifier m3() virtual {
        _;
    }
}
