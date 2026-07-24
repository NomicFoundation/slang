// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x) public pure {
        assembly {
            switch x
            case 0 { }
            case 0x0 { }
            case "" { }
            case 1 { }
            default { }
        }
    }
}
