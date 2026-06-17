// SPDX-License-Identifier: MIT
pragma solidity *;

interface ParentA {}
interface ParentB {}
interface Sub is ParentA, ParentB {}

interface ListsA is Sub, ParentA {}
