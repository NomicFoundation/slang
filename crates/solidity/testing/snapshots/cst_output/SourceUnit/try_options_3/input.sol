library SafeMath {
  function tryAdd() internal pure  {
    try 2 ++ {
      // do something
    } catch Error(string memory reason) {
      // catch failing revert() and require()
    }
  }
}
