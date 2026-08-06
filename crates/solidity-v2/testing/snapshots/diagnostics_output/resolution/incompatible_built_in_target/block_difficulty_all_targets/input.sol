// SPDX-License-Identifier: MIT
pragma solidity *;

// `block.difficulty` is available on every EVM target. From Paris onwards the
// underlying opcode was renamed to `prevrandao`, but `solc` only *warns* about
// this (8417) and still accepts the input, so slang must not reject it either.
contract C {
    function f() public view returns (uint256) {
        return block.difficulty;
    }
}
