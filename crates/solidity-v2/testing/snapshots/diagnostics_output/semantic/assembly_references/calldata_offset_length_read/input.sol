// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes calldata b) external pure {
        assembly {
            let o := b.offset
            let l := b.length
        }
    }
}
