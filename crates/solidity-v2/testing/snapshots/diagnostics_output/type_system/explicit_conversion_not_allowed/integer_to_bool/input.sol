// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bool b) public pure returns (bool, bool) {
        return (bool(1), bool(b));
    }
}
