contract Test {
  using Lib for int;

  function someFunc(int x) returns (int) {
    int here = add(x, 3);
    int lib = x.add(3);
  }

  function add(int, int32) returns (int);
}

library Lib {
  function add(int, int) returns (int);
}
