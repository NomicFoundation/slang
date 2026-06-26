// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    uint256 x;
}

contract C is Base {
    // A Yul `let` variable may not shadow a declaration visible at the assembly
    // site, including a member inherited from a base contract.
    function f() public pure {
        assembly {
            let x := 1
            pop(x)
        }
    }
}
