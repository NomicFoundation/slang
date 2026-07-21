interface IOwnable {
    function owner() external returns (address);

    // Modifiers are allowed inside interfaces until 0.8.8
    modifier onlyOwner() {
        require(this.owner() == msg.sender, "not owner");
        _;
    }
}

contract Test is IOwnable {
    function owner() external returns (address) {}

    function test() public onlyOwner {}
}
