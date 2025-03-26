uint256 constant C1 = 0;

contract X1 layout at C1 {
}

contract X2 layout at C2 { // should not bind to a constant defined in the contract
    uint256 constant C2 = 0;
}
