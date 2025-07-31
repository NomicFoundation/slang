contract TestBase {
  function add(int, int) returns (int);

  function add(int) returns (int);
}

contract Test is TestBase {
  int y;

  function someFunc(int x) returns (int) {
    int farg = this.add(x);
    int literal = super.add(3);
    int farg_2 = super.add(x, y);
    int literal_2 = this.add(3, 1);
  }
}
