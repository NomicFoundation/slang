// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `data.pop` is inert here; the call is what pops.

contract Test {
  uint[] data;

  function f() public view {
    data.pop;
  }
}
