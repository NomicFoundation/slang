// SPDX-License-Identifier: MIT
pragma solidity *;

// A struct must declare at least one member. Empty structs are reported at
// every definition location (file / contract / library / interface); structs
// with one or more members are accepted.

struct FileLevel {}
contract C { struct Inner {} }
library L { struct Inner {} }
interface I { struct Inner {} }
struct ValidFileLevel { uint x; }
contract C2 { struct Ok { uint a; uint b; } }
