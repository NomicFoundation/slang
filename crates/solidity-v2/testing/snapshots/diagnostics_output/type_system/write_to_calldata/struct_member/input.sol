// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a member of a calldata struct cannot be written to, and neither
// can an element of an array reached through one.

contract Test {
  struct Inner {
    uint value;
  }

  struct Outer {
    uint value;
    Inner inner;
    uint[] items;
  }

  function f(Outer calldata outer) external pure {
    outer.value = 1;
    outer.inner.value = 2;
    outer.items[0] = 3;
  }
}
