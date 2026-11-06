// SPDX-License-Identifier: MIT
pragma solidity *;

contract Escapes {
    function f() public pure {
        assembly {
            let s := "\q"
        }
    }
}
