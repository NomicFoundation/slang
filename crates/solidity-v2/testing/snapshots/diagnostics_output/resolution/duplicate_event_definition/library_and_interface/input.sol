// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    event Duplicated(uint256 a);
    event Duplicated(uint256 indexed a);

    event Ok(uint256 a);
    event Ok(bytes32 a);
}

interface I {
    event Duplicated(address a);
    event Duplicated(address a) anonymous;

    event Ok(address a);
    event Ok(address a, uint256 b);
}
