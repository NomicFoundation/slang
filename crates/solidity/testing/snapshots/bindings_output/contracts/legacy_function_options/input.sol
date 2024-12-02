contract RefundVault {
    function deposit() public payable {}
}
contract Test {
    RefundVault public vault;
    function test() internal {
        vault.deposit.value(msg.value)();
    }
}
