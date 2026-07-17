contract Test {
  using Lib for int;

  function someFunc(int x) public returns (int) {
    int here = add(x, 3);
    int lib = x.add(3);
  }

  function add(int, int32) public returns (int);
}

library Lib {
  function add(int, int) public returns (int) {}
}
