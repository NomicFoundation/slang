pragma solidity <0.5.0;

// Before 0.5.0 Solidity hoists all variable definitions
contract Foo {
  function bar() returns (int x) {
    //                        ^def:1
    x = y + z;
//  ^ref:1
    //      ^ref:3
    //  ^ref:2
    int y = 5;
    //  ^def:2
    {
      int z = 10;
      //  ^def:3
    }
  }
}
