// SPDX-License-Identifier: MIT
pragma solidity *;

// Both deployed functions reference B, so the entry point order decides
// which reference is reported. Slang walks the linearised function list,
// which is sorted by name, so it reaches `alpha` first. solc walks the
// external interface in declaration order and reaches `zebra` first. The
// dependency is the same either way, only the expression standing for it
// differs, so slang keeps the order the linearised list already has.

contract A {
    function zebra() public {
        new B();
    }

    function alpha() public pure returns (bytes memory) {
        return type(B).creationCode;
    }
}

contract B {
    constructor() {
        new A();
    }
}
