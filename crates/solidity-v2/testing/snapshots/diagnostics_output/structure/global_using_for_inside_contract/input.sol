// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

contract C {
    // `global` is only allowed on file-level `using` directives.
    using L for uint256 global;

    // Control: a non-global `using` directive inside a contract is allowed.
    using L for uint256;
}
