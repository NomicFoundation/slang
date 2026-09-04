// SPDX-License-Identifier: MIT
pragma solidity *;

// The alias chain crosses a public constant state variable.
contract Base {
    uint256 public constant K = 41;
}

contract C is Base {
    uint256 constant A = K;

    function f() public pure {
        assembly {
            let x := A
        }
    }
}
