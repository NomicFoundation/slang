// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 x) public pure returns (uint256 r) {
        assembly {
            // 2**256, as hex and as decimal.
            r := 0x10000000000000000000000000000000000000000000000000000000000000000
            r := 115792089237316195423570985008687907853269984665640564039457584007913129639936

            // Also as an argument and as a switch case value.
            r := add(x, 0x10000000000000000000000000000000000000000000000000000000000000000)
            // Yul has no constant folding: each literal is checked on its own,
            // even when the result of the operation would fit.
            r := sub(0x10000000000000000000000000000000000000000000000000000000000000001, 0x10000000000000000000000000000000000000000000000000000000000000000)

            switch x
            case 0x10000000000000000000000000000000000000000000000000000000000000000 { }
            default { }
        }
    }
}
