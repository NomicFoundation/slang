// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    error Err(uint);
}

contract B is A {
    error Err(bytes32);
}
