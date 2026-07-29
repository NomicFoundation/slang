// SPDX-License-Identifier: MIT
pragma solidity *;

// Overriding an `external` function may change the data location of both
// parameters and return values, so every override below is recognised and `B`
// does not need to be abstract.
abstract contract A {
    struct S {
        uint256 x;
    }

    // A single reference-type return value.
    function f(uint256[] calldata a)
        external
        pure
        virtual
        returns (uint256[] calldata);

    // A tuple mixing a reference type with a value type: the value type still
    // has to match exactly, since it carries no data location.
    function g(uint256[] calldata a)
        external
        pure
        virtual
        returns (uint256[] calldata, uint256);

    // A tuple where the override *swaps* both locations, so the relaxation has
    // to apply per element and in both directions.
    function h(int256[] calldata b)
        external
        pure
        virtual
        returns (uint256[] calldata, int256[] memory);

    // Nested arrays: the inner element type carries a location of its own.
    function i(uint256[][] calldata c)
        external
        pure
        virtual
        returns (uint256[][] calldata);

    // Structs, compared by definition and location.
    function j(S calldata s) external pure virtual returns (S calldata);

    // Fixed-size arrays, which also compare their length.
    function k(uint256[2] calldata d)
        external
        pure
        virtual
        returns (uint256[2] calldata);

    // `bytes` and `string`.
    function l(bytes calldata e, string calldata t)
        external
        pure
        virtual
        returns (bytes calldata, string calldata);
}

contract B is A {
    function f(uint256[] memory a)
        public
        pure
        override
        returns (uint256[] memory)
    {
        return a;
    }

    function g(uint256[] memory a)
        public
        pure
        override
        returns (uint256[] memory, uint256)
    {
        return (a, a.length);
    }

    function h(int256[] calldata b)
        public
        pure
        override
        returns (uint256[] memory, int256[] calldata)
    {
        return (new uint256[](0), b);
    }

    function i(uint256[][] memory c)
        public
        pure
        override
        returns (uint256[][] memory)
    {
        return c;
    }

    function j(S memory s) public pure override returns (S memory) {
        return s;
    }

    function k(uint256[2] memory d)
        public
        pure
        override
        returns (uint256[2] memory)
    {
        return d;
    }

    function l(bytes memory e, string memory t)
        public
        pure
        override
        returns (bytes memory, string memory)
    {
        return (e, t);
    }
}
