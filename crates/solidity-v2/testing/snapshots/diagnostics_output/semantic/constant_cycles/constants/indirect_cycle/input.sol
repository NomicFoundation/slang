// SPDX-License-Identifier: MIT
pragma solidity *;

// `A` is itself acyclic (`A = C + 1`) but transitively depends on the `C <-> D`
// cycle, so using it as an array size must still be rejected — the cycle is
// caught by the constant-evaluation recursion limit, like solc.
contract Z {
    uint256 constant C = D;
    uint256 constant D = C;
    uint256 constant A = C + 1;
    uint256[A] data;
}
