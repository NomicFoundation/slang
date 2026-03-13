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
