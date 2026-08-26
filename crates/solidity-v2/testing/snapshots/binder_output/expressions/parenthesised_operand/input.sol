contract TestBase {
  function g() public pure virtual returns (uint) {
    return 1;
  }
}

contract Test is TestBase {
  function g() public pure override returns (uint) {
    return 2;
  }

  function parenthesised() public pure {
    // Parentheses are transparent, so each pair below resolves identically:
    // the parenthesised operand keeps the typing of the expression it wraps.
    super.g();
    (super).g();

    abi.encode(uint(1));
    (abi).encode(uint(1));
  }
}
