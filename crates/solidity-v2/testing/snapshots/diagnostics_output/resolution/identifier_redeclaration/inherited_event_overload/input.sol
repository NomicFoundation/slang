// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    event E(uint);
}

contract B is A {
    event E(bytes32);
}
