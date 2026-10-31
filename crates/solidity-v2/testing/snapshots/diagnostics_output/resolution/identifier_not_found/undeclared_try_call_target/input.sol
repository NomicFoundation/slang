// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {
        try X(address(0)).g() {} catch {}
    }
}
