contract Test {
  function someFunc(int64 x) public returns (int256) {
    return add(add(x), add(3));
  }

  function add(int64, int64) public returns (int256) {}

  function add(int64) public returns (int64) {}
}
