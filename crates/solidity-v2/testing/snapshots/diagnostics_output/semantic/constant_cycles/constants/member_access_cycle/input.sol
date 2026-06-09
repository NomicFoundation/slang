// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    uint256 internal constant A = X;
}

uint256 constant X = L.A;
