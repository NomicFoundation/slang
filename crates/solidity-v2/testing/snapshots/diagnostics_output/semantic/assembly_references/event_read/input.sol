// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event Ev();

    function f() public pure {
        assembly {
            let t := Ev
        }
    }
}
