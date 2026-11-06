// SPDX-License-Identifier: MIT
pragma solidity *;

// The types argument of `abi.decode` accepts type names, but not inside an
// array literal, which still needs values.

contract C {
    function f() public pure {
        abi.decode("", ([uint][2]));
    }
}
