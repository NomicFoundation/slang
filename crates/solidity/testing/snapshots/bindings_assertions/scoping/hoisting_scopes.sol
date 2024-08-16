contract Foo {
  function bar() returns (int x) {
    //                        ^def:1
    x = y + z;
    //<ref:1
    //      ^ref:3 (<0.5.0)
    //      ^ref:! (>=0.5.0)
    //  ^ref:2 (<0.5.0)
    //  ^ref:! (>=0.5.0)
    int y = 5;
    //  ^def:2
    {
      int z = 10;
      //  ^def:3
    }
  }
}
