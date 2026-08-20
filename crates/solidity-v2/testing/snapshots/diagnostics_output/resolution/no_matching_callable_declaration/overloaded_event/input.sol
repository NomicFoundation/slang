// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    event E(uint256 u);

    event E(bool b);

    function no_overload_accepts() internal {
        // A string matches neither event's parameter.
        emit E("nope");
    }

    function one_overload_accepts() internal {
        emit E(true);
    }
}
