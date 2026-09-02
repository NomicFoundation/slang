// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint[] storage a) internal {
        assembly {
            let t := a.length
        }
    }
}
