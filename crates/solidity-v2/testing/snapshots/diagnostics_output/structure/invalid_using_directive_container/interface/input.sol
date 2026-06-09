// SPDX-License-Identifier: MIT
pragma solidity *;

library L {}

interface I {
    using L for uint256;
}
