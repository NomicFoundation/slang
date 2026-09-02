// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        function() external fp;
        assembly {
            let t := fp.offset
        }
    }
}
