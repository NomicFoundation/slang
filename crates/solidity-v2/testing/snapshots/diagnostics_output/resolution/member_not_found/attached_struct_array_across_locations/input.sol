// SPDX-License-Identifier: MIT
pragma solidity *;

// A storage array of structs attaches to a function taking it in memory: only
// the data location of the array and its elements differs.
library L {
    struct S {
        int104 v;
    }

    function cur(S[2] memory c) internal pure returns (int104) {
        return c[0].v;
    }
}

contract C {
    using L for L.S[2];

    L.S[2] s;

    function test() internal view returns (int104) {
        return s.cur();
    }
}
