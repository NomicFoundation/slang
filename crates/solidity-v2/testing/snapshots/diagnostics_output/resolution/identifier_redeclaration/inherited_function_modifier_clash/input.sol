// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function m(uint a) public {}
}

contract B is A {
    modifier m(uint a) {
        _;
    }
}
