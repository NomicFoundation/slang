// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: both operands of `&&` and `||` are value positions.

contract Test {
  function f() public view {
    bool flaggedAnd = abi && tx;
    bool flaggedOr = block || msg;
  }
}
