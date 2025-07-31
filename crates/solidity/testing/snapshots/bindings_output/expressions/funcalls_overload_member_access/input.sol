contract Overloaded {
  function add(int64, int64) returns (Overloaded);

  function add(int64) returns (Overloaded);
}

contract Test {
  function someFunc(Overloaded x, int64 y) public {
    x.add(y).add(y, y);
  }
}
