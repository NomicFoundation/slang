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
