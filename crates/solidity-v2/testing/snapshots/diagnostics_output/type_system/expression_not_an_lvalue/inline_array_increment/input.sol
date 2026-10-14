// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public {
        // https://github.com/argotorg/solidity/blob/v0.8.0/test/libsolidity/syntaxTests/inline_arrays/lvalues_as_inline_array.sol
        // An inline array is not an lvalue, so it cannot be incremented.
        [1, 2, 3]++;
    }
}
