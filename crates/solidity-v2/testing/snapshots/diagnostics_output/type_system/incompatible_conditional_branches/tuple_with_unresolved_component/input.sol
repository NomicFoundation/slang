// SPDX-License-Identifier: MIT
pragma solidity *;

contract Test {
  function f(bool c) public view returns (uint, uint) {
    // Reported once, where `super` is used as a value: a tuple with a
    // component that has no type is not judged again by the conditional.
    return c ? (1, super) : (1, 2);
  }
}
