contract Test {
    struct Funder {
        address addr;
        uint amount;
    }

    function buildFunder() public payable returns (Funder memory) {
        return Funder({addr: msg.sender, amount: msg.value});
    }
}
