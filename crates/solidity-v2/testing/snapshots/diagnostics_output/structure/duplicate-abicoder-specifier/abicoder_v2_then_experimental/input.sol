// SPDX-License-Identifier: MIT
pragma solidity *;

// Solc has a bug where it reports an error on `experimental ABIEncoderV2` after `abicoder v2`, but not the other way around:
pragma abicoder v2;
pragma experimental ABIEncoderV2;
