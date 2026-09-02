// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(function() external fp) public pure {
        assembly {
            let t := fp
        }
    }
}
