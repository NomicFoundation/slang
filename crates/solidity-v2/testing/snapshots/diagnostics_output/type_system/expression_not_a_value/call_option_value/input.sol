// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a call option is passed to the call, so it is a value.

interface Target {
  function g() external payable;
}

contract Test {
  function f(Target target) public {
    target.g{value: abi}();
  }
}
