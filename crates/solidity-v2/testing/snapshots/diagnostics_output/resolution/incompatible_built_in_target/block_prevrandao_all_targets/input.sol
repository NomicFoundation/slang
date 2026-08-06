// SPDX-License-Identifier: MIT
pragma solidity *;

// `block.prevrandao` is accepted on every EVM target: from Paris on it maps to
// the opcode of the same name, and before Paris `solc` only *warns* (9432) that
// it will be treated as `difficulty`, still compiling the input.
contract C {
    function f() public view returns (uint256) {
        return block.prevrandao;
    }
}
