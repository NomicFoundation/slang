// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the first member that does not resolve is reported.
contract A {}

contract C {
    function f() public pure {
        revert A.B.Err();
    }
}
