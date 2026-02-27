// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/tuple_assignment_empty/input.sol
library SafeMath {
  function tryAdd() internal pure  {
    // If we don't use the return, V1 will parse them as TupleDeconstructionStatement
    return () = foo;
    return (,) = foo;
    return (,,) = foo;

    () = foo;
    (,) = foo;
    (,,) = foo;
  }
}

// <<<
