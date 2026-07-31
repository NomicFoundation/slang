// SPDX-License-Identifier: MIT
pragma solidity *;

// `Int.wrap(1) + Int.wrap(2)` executes `add`, which embeds C's own creation
// code, so this is a real cycle, reported at the creation code access the
// operator function body reaches.

type Int is int256;

function add(Int, Int) pure returns (Int) {
    bytes memory b = type(C).creationCode;
    return Int.wrap(int256(uint256(b.length)));
}

using {add as +} for Int global;

contract C {
    function f() public pure returns (Int) {
        return Int.wrap(1) + Int.wrap(2);
    }
}
