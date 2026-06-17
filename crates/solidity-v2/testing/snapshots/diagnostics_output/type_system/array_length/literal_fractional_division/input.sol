// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // All-literal division stays an exact rational (`10 / 4` is `5/2`), which
    // is not a valid (integer) length.
    uint256[10 / 4] x;
}
