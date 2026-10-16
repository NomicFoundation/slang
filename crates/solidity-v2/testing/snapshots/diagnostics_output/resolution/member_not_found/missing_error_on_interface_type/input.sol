// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    error CustomError();
}

contract C {
    function f() public pure {
        revert I.CustomEror();
    }
}
