// SPDX-License-Identifier: MIT
pragma solidity *;

// The suffixes exist from 0.8.10, so earlier versions reject them.
contract C {
    function g() external {}

    function f() public view {
        function() external fp = this.g;
        assembly {
            let s := fp.selector
            let a := fp.address
        }
    }
}
