// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x) public pure {
        assembly {
            switch x
            case 1 { }
            case 2 { }
            case 0x3 { }
            default { }
        }
    }
}
