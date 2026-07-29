// SPDX-License-Identifier: MIT
pragma solidity *;

// The data-location relaxation for `external` overrides must not extend to the
// underlying type: `int256[]` is not `uint256[]` whatever its location, so this
// does not override and `B` should be abstract.
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
