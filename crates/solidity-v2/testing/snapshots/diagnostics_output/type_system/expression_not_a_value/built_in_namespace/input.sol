// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `abi` is a namespace, so it has no value to assign.

contract Test {
  function f() public pure {
    uint flagged = abi;
  }
}
