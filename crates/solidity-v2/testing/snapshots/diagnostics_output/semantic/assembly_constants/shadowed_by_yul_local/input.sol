// SPDX-License-Identifier: MIT
pragma solidity *;

// The reference resolves to the Yul local, so no constant diagnostic is
// emitted. Only the shadowing declaration itself is reported.
bytes32 constant K = keccak256("x");

contract C {
    function f() public pure {
        assembly {
            let K := 1
            let x := K
        }
    }
}
