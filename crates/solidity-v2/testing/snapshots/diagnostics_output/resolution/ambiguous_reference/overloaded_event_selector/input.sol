// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 u);

    event E(bool b);

    event F(uint256 u);

    function ambiguous() internal pure returns (bytes32) {
        // The operand of the member access is a bare name in value position,
        // and an event is not a variable.
        return E.selector;
    }

    function unambiguous() internal pure returns (bytes32) {
        return F.selector;
    }
}
