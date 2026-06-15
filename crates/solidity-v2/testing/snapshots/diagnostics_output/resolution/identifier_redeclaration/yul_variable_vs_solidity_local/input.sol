// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint x;
        assembly {
            // Yul variables cannot shadow declarations outside the
            // assembly block.
            let x := 1
        }
    }
}
