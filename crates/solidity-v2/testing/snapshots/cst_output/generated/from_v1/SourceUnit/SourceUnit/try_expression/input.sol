// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/try_expression/input.sol
library SafeMath {
  function tryAdd() internal pure  {
    try 2 ++ {
      // do something
    } catch Error(string memory reason) {
      // catch failing revert() and require()
    }
  }
}
// <<<
