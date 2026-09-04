// SPDX-License-Identifier: MIT
pragma solidity *;

// Readability is checked before forwardness, so a forward referenced value
// that is not a direct number is reported as unsupported.
contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}

uint256 constant K = uint256(1) + 1;
