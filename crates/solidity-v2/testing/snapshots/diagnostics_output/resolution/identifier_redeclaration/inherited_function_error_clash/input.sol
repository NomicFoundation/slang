// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function Err() public pure {}
}

contract B is A {
    error Err();
}
