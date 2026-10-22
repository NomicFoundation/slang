// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m(uint a) {
        _;
    }
}

contract B is A {
    function m(uint a) public {}
}
