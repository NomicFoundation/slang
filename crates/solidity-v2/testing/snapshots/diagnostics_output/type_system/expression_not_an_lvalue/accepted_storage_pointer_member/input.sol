// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a call may yield a storage pointer, whose members are locations.

contract Test {
  struct Pair { uint left; uint right; }

  Pair pair;

  function pointer() internal view returns (Pair storage) {
    return pair;
  }

  function f() internal {
    pointer().left = 1;
  }
}
