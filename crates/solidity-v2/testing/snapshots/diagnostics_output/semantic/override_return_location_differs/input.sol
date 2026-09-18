// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f(uint256[] calldata a)
        public
        pure
        virtual
        returns (uint256[] calldata)
    {
        return a;
    }
}

// A function overriding a non-external one has to keep the data locations of
// its return variables. This is enforced from 0.8.14 on.
contract B is A {
    function f(uint256[] calldata a)
        public
        pure
        override
        returns (uint256[] memory)
    {
        return a;
    }
}
