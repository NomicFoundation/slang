// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/try_options/input.sol
library SafeMath {
  function tryAdd() internal pure  {
    try foo{gas: 123}() {
      // do something
    } catch Error(string memory reason) {
      // catch failing revert() and require()
    }
  }
}

// <<<
