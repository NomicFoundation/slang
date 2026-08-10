// SPDX-License-Identifier: MIT
pragma solidity *;

interface IBase {
    event FromInterface(uint256 a);
}

contract Base {
    event FromBase(uint256 a);
    event Ok(uint256 a);
}

// An event sharing an ABI slot with an inherited one cannot be told apart from
// it, even though the two are declared in different contracts.
contract Derived is Base, IBase {
    event FromBase(uint256 indexed a);
    event FromInterface(uint256 a) anonymous;
    event Ok(uint256 a, address b);
}

// Two bases can also clash with each other, in a contract declaring neither.
contract LeftParent {
    event FromParents(uint256 a, uint256 b);
}

contract RightParent {
    event FromParents(uint256 a, uint256 indexed b);
}

contract Diamond is LeftParent, RightParent {}
