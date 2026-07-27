// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {}

// A library is not allowed to declare an inheritance list.
library L is Base {}
