contract Test {
  function foo() public private returns (uint) {
    return 1;
  }

  function bar() private public returns (uint) {
    return 2;
  }

  function baz() public pure private returns (uint) {
    return 3;
  }

  function qux() internal external returns (uint) {
    return 4;
  }

  function quux() private public internal returns (uint) {
    return 5;
  }

  function separated1() public pure view private returns (uint) {
    return 6;
  }

  function separated2() public view payable private returns (uint) {
    return 7;
  }

  function separated3() external pure payable view internal returns (uint) {
    return 8;
  }

  function separated4() public pure view external returns (uint) {
    return 9;
  }

  function triple1() public private internal returns (uint) {
    return 10;
  }

  function triple2() external internal private returns (uint) {
    return 11;
  }

  function triple3() public private external internal returns (uint) {
    return 12;
  }
}
