contract C {
    receive() external payable virtual returns (uint256) {}
}
contract D is C {
    receive() external payable override {}
}
