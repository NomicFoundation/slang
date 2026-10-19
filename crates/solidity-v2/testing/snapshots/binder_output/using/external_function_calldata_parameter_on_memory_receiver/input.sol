pragma solidity *;

library D {
    function f(bytes calldata) external pure {}
}

contract C {
    using D for bytes;

    function g(bytes memory x) public view {
        // An external function receives its first argument through the ABI,
        // which copies memory into calldata.
        x.f();
    }
}
