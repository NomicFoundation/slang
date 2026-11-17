// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes memory x) public pure {
        string.concat(x);
    }
}
