// --- path: declarations.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

function f(uint256 a) pure returns (uint256) {
    return a;
}

function g(uint256 a) pure returns (uint256) {
    return a + 1;
}

// --- path: consumer.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

// A qualified import binds one name to the whole file, so the declarations it
// holds never join the importing file's overload sets — not even the two
// indistinguishable ones below.
import "./declarations.sol" as ns;
import {f as aliased} from "./declarations.sol";

contract C {
    function callThem() public pure returns (uint256) {
        return ns.f(1) + ns.g(2) + aliased(3);
    }
}
