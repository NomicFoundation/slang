contract RefundVault {
    function deposit() public payable;
    function deposit2() payable; // valid in Solidity < 0.5.0
}
contract Test {
    function test(RefundVault vault) internal {
        vault.deposit.value(1)();
        vault.deposit2.value(1)();
    }
}
