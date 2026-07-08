// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function x() external view returns (uint);
}

// The public state variable's getter implements the interface function, so `C`
// is concrete and need not be `abstract`.
contract C is I {
    uint public override x;
}
