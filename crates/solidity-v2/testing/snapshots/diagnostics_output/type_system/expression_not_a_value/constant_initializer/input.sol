// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a constant's initialiser is a value position, at file level and
// inside a contract alike.

uint constant FLAGGED_FILE = abi;

contract Test {
  uint constant FLAGGED_MEMBER = tx;
}
