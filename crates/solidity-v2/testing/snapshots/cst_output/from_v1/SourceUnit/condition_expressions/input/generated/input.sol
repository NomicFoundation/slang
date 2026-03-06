// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/condition_expressions/input.sol
contract Test {
  function foo() public {
    bool a = false;
    bool b = true;
    bool c = true;
    bool d = false;
    bool e = true;

    // Ternary right-associativity
    a ? b : c ? d : e; // (a ? b : (c ? d : e))
    a ? b ? c : d : e; // (a ? (b ? c : d) : e)
  }
}

// <<<
