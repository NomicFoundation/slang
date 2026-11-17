// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint256 r) {
        assembly {
            // 2**256 - 1, as hex and as decimal.
            r := 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            r := 115792089237316195423570985008687907853269984665640564039457584007913129639935

            // Leading zeros of a hex literal do not count towards its size.
            r := 0x00000000000000000000000000000000000000000000000000000000000000000001
        }
    }
}
