// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // NOTE: solc only reports the first invalid function type, since the error
    // is fatal there. Slang reports all of them.
    function (uint256) private x;
    function (uint256) public y;
}
