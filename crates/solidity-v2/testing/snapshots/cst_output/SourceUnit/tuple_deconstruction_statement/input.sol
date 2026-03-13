// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/tuple_deconstruction_statement/input.sol
library SafeMath {
  function tryAdd() internal pure  {
    (, bool z, ) = foo;
    (bool a, uint b) = foo;
    (uint x, , uint y) = foo;
    (,,,,bool a) = foo;
  }
}

// <<<
