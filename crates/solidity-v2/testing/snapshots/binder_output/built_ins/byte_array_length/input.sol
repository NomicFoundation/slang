pragma solidity *;

contract C {
    function g(uint8) internal pure {}
    function g(bool) internal pure {}

    function f(bytes32 b) internal pure {
        // The length of a fixed-size byte array is a `uint8`.
        g(b.length);
    }
}
