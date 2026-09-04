// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E {
        A
    }

    function f() public pure {
        assembly {
            let t := E.A
        }
    }
}
