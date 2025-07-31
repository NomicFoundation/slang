contract TestBase {
  function add(int, int) public returns (int);

  function add(int) returns (int);
}

contract Test {
  int y;

  function someFunc(TestBase[] arr, int x) returns (int) {
    arr[0].add(x);
    arr[0].add(x, y);
  }
}
