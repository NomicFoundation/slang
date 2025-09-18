contract Test {
  int y;

  function someFunc(int x) returns (int) {
    int farg = add(x);
    int literal = add(3);
  }

  function add(int) returns (int);

  function add(int) returns (int);
}
