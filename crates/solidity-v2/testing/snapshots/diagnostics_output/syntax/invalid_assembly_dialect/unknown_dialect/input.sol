// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function unknownDialect() public pure {
        // `evmasm` is the only dialect an assembly statement can name.
        assembly "evm" {}
    }
}
