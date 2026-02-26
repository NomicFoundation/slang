contract Counter {
  uint _count;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta) public returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
}
