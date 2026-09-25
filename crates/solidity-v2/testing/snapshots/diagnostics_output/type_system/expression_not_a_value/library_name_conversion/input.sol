// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a conversion takes a type name, since a library name converts to
// the address the library is deployed at.

library L {
  function f() external {}
}

contract Test {
  function f() public pure returns (address) {
    return address(L);
  }
}
