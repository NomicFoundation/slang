// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[] memory a) public pure {
        // Indexing an array value needs an index expression.
        a[];

        // Omitting the index of a type name is fine, it names an array type.
        abi.decode("", (uint256[]));
    }
}
