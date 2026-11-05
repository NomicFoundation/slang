// SPDX-License-Identifier: MIT
pragma solidity *;

library D {
    function f(uint256[] storage) public view {}
}

contract C {
    using D for uint256[];

    function g(uint256[] memory x) public view {
        // A storage parameter needs a storage receiver, even for a function
        // called through the ABI.
        x.f();
    }
}
