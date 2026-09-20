// SPDX-License-Identifier: MIT
pragma solidity *;

// `block.prevrandao` requires 0.8.18, which is also the first version whose
// default EVM target is Paris — so at its own default target every version
// either doesn't know the member or backs it with the opcode of the same name.
// Its companion `block_prevrandao_pre_paris` covers the remaining case, where a
// compiler that knows the member targets a pre-Paris EVM.
contract C {
    function f() public view returns (uint256) {
        return block.prevrandao;
    }
}
