// Emitting events introduced syntactically in 0.4.21 but `emit` was usable as identifier until 0.5.0

contract ClientReceipt {
    event Deposit();
    function deposit() public payable {
        uint256 emit;
        emit Deposit();
    }
}
