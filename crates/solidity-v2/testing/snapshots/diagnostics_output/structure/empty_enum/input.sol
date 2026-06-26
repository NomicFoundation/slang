// SPDX-License-Identifier: MIT
pragma solidity *;

// An enum must declare at least one member. The parser accepts an empty
// member list, so this is reported as a structural diagnostic at every
// definition location (file / contract / library / interface).

enum FileLevel {}
contract C { enum Inner {} }
library L { enum Inner {} }
interface I { enum Inner {} }
