// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function ext() external {}

    // A Yul variable may shadow a same-contract `external` function: external
    // functions are not visible in the internal (assembly) scope, so this is
    // not a redeclaration. solc accepts it.
    function f() public pure {
        assembly {
            let ext := 1
            pop(ext)
        }
    }
}
