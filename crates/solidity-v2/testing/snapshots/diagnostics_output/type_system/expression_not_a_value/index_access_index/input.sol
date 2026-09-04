// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an index is a value position, as are a slice's bounds. The
// indexed operand is a statement here, which the index does not become.

contract Test {
  uint[] data;

  function f(bytes calldata input) public view {
    data[abi];
    input[tx:];
  }
}
