// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: `payable(this).transfer` is inert here; the call is what transfers.

contract Test {
  receive() external payable {}

  function f() public view {
    payable(this).transfer;
  }
}
