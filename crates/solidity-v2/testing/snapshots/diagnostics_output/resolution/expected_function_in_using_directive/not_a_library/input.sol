// SPDX-License-Identifier: MIT
pragma solidity *;

contract Other {}

contract C {
    using Other for uint256;
}
