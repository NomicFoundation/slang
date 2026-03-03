// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// >>> Copied from crates/solidity/testing/snapshots/cst_output/SourceUnit/condition_and_assignment_expressions/input.sol
contract Test {
  function foo() public {
    bool a = false;
    bool b = true;
    bool c = true;
    bool d = false;
    bool e = true;

    a = b ? c : d; // (a = (b ? c : d))
    a ? b = c : d; // (a ? (b = c) : d)
    a ? b : c = d; // (a ? b : (c = d))
  }
}

// <<<
