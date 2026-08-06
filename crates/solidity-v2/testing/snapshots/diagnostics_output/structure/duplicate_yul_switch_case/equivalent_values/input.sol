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

    function g(uint256 x) public pure {
        assembly {
            switch x
            case 64 { }
            case 0x40 { }
            case 65 { }
            default { }
        }
    }

    function h(uint256 x) public pure {
        assembly {
            switch x
            case 0x4100000000000000000000000000000000000000000000000000000000000000 { }
            case "A" { }
            case "ABC" { }
            default { }
        }
    }
}
