// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/layout_at/input.sol
contract SafeMath layout at 123  {
}

contract SafeMath layout at foo{gas:123}  {
}

contract SafeMath layout at foo()  {
}

contract SafeMath layout at 123 + foo  {
}

// <<<
