// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/tuple_assignment/input.sol
library SafeMath {
  function tryAdd() internal pure {
    // If we don't use the return, V1 will parse them as TupleDeconstructionStatement
    return (, z, ) = foo;
    return (a, b) = foo;
    return (x, , y) = foo;
    return (, , , , a) = foo;
    return (a, ) = foo;

    (, z, ) = foo;
    (a, b) = foo;
    (x, , y) = foo;
    (, , , , a) = foo;
    (a, ) = foo;
  }
}
// <<<
