// SPDX-License-Identifier: MIT
pragma solidity *;

// An unimplemented (abstract) function cannot have modifier invocations.
abstract contract A {
    modifier onlyOwner() {
        _;
    }

    function foo() public virtual onlyOwner;
    function bar() public onlyOwner virtual;

    // A fully-implemented function may have modifier invocations, and must not
    // be flagged.
    function baz() public onlyOwner {}
}
