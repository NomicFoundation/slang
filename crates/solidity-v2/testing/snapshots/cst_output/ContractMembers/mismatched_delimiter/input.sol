contract C {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/ContractMembers/mismatched_delimiter/input.sol
function someFunc() public {
    {
        uint256 arg = (1 + 2;
    })
}

// <<<
}
