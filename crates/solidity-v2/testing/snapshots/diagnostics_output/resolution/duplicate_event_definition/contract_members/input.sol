// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Distinguishable overloads: the parameter lists differ.
    event Ok(uint256 a);
    event Ok(uint256 a, address b);
    event Ok(string a);

    // Identical parameter lists.
    event Duplicated(uint256 a);
    event Duplicated(uint256 b);

    // `indexed` places a parameter in a topic rather than in the data, but it
    // doesn't change its type, so it doesn't tell two events apart.
    event IgnoresIndexed(uint256 a);
    event IgnoresIndexed(uint256 indexed a);

    // Neither does dropping the event's own topic.
    event IgnoresAnonymous(address a);
    event IgnoresAnonymous(address a) anonymous;
}
