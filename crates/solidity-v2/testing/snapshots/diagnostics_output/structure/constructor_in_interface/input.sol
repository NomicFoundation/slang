// SPDX-License-Identifier: MIT
pragma solidity *;

// A constructor with a body parses fine, but is not allowed inside an
// interface. solc reports TypeError 6482; slang emits
// structure/constructor-in-interface. (A bodyless constructor would instead be
// a parse error, so a body is required to reach this semantic check.)
interface I {
    constructor() {}
}
