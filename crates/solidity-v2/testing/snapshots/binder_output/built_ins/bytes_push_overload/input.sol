pragma solidity *;

contract C {
    bytes s;

    function g(bytes1) internal pure {}

    function g(bool) internal pure {}

    function f() internal {
        g(s.push());
    }
}
