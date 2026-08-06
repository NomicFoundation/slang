// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256 n) public pure {
        assembly {
            // Functions are allowed everywhere except a for-loop init block.
            function noop() {}
            for { } n { function inPost() {} inPost() } {
                function inBody() {}
                inBody()
                noop()
            }

            // They're allowed in a post block nested inside an init block.
            for { for { } n { function g() {} } { } } n { } { }
        }
    }
}
