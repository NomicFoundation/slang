contract Other {
  function overloaded(int x) public payable returns (uint) {
    return 42;
  }

  function overloaded() payable returns (uint) {
    return 0;
  }
}

contract Test {
  function someFunc(Other x, int y) {
    x.overloaded{ value: 10, gas: 1000 }();
    x.overloaded{ value: 10, gas: 1000 }(y);
  }
}
