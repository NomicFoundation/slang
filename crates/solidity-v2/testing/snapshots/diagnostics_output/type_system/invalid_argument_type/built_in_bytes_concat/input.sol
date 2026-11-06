// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x) public pure {
        bytes.concat(x);
    }
}
