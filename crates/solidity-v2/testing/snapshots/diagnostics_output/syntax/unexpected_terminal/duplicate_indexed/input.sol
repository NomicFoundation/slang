// SPDX-License-Identifier: MIT
pragma solidity *;

contract c {
    // https://github.com/argotorg/solidity/blob/v0.8.18/test/libsolidity/syntaxTests/events/event_multiple_indexed_of_one_parameter.sol
    // An event parameter can only be marked `indexed` once.
    event e1(uint indexed indexed a);
}
