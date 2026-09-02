// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint[] storage a) internal pure {
        assembly {
            let s := a.slot
        }
    }
}
