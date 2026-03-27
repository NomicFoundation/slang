contract Test {
  function someFunc(int64 x) returns (int256) {
    return add(add(x), add(3));
  }

  function add(int64, int64) returns (int256);

  function add(int64) returns (int64);
}
