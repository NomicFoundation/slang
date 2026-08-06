// SPDX-License-Identifier: MIT
pragma solidity *;

// `block.prevrandao` requires 0.8.18: before that the member does not exist and
// `solc` reports a genuine error. From 0.8.18 on it is accepted even on a
// pre-Paris target, where `solc` only *warns* (9432) that it will be treated as
// `difficulty` — so slang must not reject it there.
contract C {
    function f() public view returns (uint256) {
        return block.prevrandao;
    }
}
