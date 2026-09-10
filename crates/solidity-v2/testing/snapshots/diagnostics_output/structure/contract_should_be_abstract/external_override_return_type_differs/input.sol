// SPDX-License-Identifier: MIT
pragma solidity *;

// Return types play no part in deciding that `B.f` implements `A.f`, so `B` is
// concrete. The `int256[]` return type is reported by the override check
// instead.
abstract contract A {
    function f(uint256[] calldata a)
        external
        pure
        virtual
        returns (uint256[] calldata);
}

contract B is A {
    function f(uint256[] memory a)
        public
        pure
        override
        returns (int256[] memory)
    {
        return new int256[](a.length);
    }
}
