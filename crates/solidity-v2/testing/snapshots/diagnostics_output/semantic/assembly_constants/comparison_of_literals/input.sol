// SPDX-License-Identifier: MIT
pragma solidity *;

// The value is built only from literals, but a comparison produces a typed
// bool, not an untyped literal, so it is not a direct number constant.
bool constant K = 1 < 2;

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
