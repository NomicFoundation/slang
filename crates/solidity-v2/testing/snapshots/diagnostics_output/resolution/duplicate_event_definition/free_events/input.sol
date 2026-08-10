// SPDX-License-Identifier: MIT
pragma solidity *;

// Distinguishable file-level overloads.
event Ok(uint256 a);
event Ok(uint256 a, address b);

// Identical parameter lists, `indexed` aside.
event Duplicated(uint256 a);
event Duplicated(uint256 indexed a);

contract C {
    // A contract member lives in its own namespace, so it never clashes with a
    // file-level declaration.
    event Duplicated(uint256 a);

    function emitIt() public {
        emit Ok(1);
    }
}
