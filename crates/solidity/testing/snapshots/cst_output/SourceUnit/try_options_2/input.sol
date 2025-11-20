library SafeMath {
  function tryAdd() internal pure  {
    try foo {
      // do something
    } catch Error(string memory reason) {
      // catch failing revert() and require()
    }
  }
}
