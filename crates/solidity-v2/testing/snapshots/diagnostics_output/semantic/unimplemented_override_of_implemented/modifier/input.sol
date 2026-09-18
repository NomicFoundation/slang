// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m() virtual {
        _;
    }
}

// An implemented modifier cannot be overridden by an unimplemented one. The
// rule applies to modifiers from 0.8.5 on.
abstract contract B is A {
    modifier m() virtual override;
}
