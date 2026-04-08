contract Test {
  int y;

  function someFunc(int x) returns (int) {
    int farg = add(x);
    int literal = add(3);
    int farg_2 = add(x, y);
    int literal_2 = add(3, 1);
  }

  function add(int, int) returns (int);

  function add(int) returns (int);
}
