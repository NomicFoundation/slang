// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        assembly {
            let t := C.slot
        }
    }
}
