contract Foo {
  // should resolve all correctly for < 0.5.0
  function bar() returns (int x) {
    x = y + z;
    int y = 5;
    {
      int z = 10;
    }
  }
}
