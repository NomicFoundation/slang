// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/ContractDefinition/emit_contextual/input.sol
// Emitting events introduced syntactically in 0.4.21 but `emit` was usable as identifier until 0.5.0

contract ClientReceipt {
    event Deposit();
    function deposit() public payable {
        uint256 emit;
        emit Deposit();
    }
}
// <<<
