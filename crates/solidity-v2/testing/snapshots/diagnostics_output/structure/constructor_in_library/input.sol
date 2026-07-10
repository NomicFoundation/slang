// SPDX-License-Identifier: MIT
pragma solidity *;

// A constructor with a body parses fine, but is not allowed inside a library.
// solc reports TypeError 7634; slang emits structure/constructor-in-library.
// (A bodyless constructor would instead be a parse error, so a body is
// required to reach this semantic check.)
library L {
    constructor() {}
}
