library SafeMath {
  function tryAdd() internal pure  {
    try foo{gas: 123}() {
      // do something
    } catch Error(string memory reason) {
      // catch failing revert() and require()
    }
  }
}
