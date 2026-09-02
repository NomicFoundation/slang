// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a variable, a state variable, a struct member, an array element
// and a mapping value are all locations.

contract Test {
  struct Pair { uint left; uint right; }

  uint stateValue;
  uint[] items;
  bytes data;
  mapping(uint => uint) values;
  Pair pair;

  function f(uint parameter) internal {
    uint local;
    local = parameter;
    stateValue = local;
    items[0] = 1;
    data[0] = 0x01;
    values[0] = 1;
    pair.left = 1;
  }
}
