// SPDX-License-Identifier: MIT
pragma solidity *;

// `chainid` was introduced in Istanbul, but its name is reserved on *every* EVM
// target: solc rejects the declaration below even when targeting Petersburg,
// where the opcode does not exist. Reserving Yul built-in names only while they
// are available would wrongly accept this.
contract C {
    function f() public pure {
        assembly {
            let chainid := 1
        }
    }
}
