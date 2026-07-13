contract Test {
  int y;

  function someFunc(int x) public returns (int) {
    int farg = add(x);
    int literal = add(3);
  }

  function add(int) public returns (int);

  function add(int) public returns (int);
}
