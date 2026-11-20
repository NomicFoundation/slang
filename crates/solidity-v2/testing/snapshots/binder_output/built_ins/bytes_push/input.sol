pragma solidity *;

contract C {
    bytes data;

    function g(bytes1) internal pure {}
    function g(uint8) internal pure {}

    function f() internal {
        // Pushing onto `bytes` yields a reference to a `bytes1` element.
        g(data.push());
    }
}
