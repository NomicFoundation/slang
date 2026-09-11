// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a built-in takes a type name, since the second argument of
// `abi.decode` is the tuple of types to decode into.

contract Test {
  function f(bytes memory data) public pure returns (uint256, bool) {
    return abi.decode(data, (uint256, bool));
  }
}
