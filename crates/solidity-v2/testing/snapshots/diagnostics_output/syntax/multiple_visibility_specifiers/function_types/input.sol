contract Test {
  function foo(function() public private returns (uint) callback) {}
  function bar(function() internal external returns (uint) cb1, function() private public returns (uint) cb2) {}
  function baz(function() public pure private returns (uint) callback) {}
  function triple1(function() public private internal returns (uint) cb) {}
  function triple2(function() external internal private returns (uint) cb) {}
}
