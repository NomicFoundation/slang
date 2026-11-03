// SPDX-License-Identifier: MIT
pragma solidity *;

contract Strings {
    function f() public pure {
        assembly {
            let s := 'naïve'
        }
    }
}
