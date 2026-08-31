// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E {
        A,
        B
    }

    E constant K = E.A;

    function f() public pure {
        assembly {
            let x := K
        }
    }
}
