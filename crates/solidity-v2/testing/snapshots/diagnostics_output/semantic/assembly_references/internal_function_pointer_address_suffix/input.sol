// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() internal pure {}

    function f() public pure {
        function() internal pure fp = g;
        assembly {
            let t := fp.address
        }
    }
}
