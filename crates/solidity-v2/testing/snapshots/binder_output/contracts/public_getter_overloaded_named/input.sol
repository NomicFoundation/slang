pragma solidity *;

library L {
    function balances(C) internal pure {}
}

contract C {
    mapping(address owner => uint256) public balances;
}

contract D {
    using L for C;

    // Only the getter accepts an argument besides the receiver.
    function test(C c) public view {
        c.balances({owner: address(0)});
    }
}
