library Id {
  enum Answer {
    Yes
  }

  function id(Answer ans) public returns (Answer) {
    return ans;
  }
}

contract Test {
  using Id for Id.Answer;

  function testFunc() public returns (Id.Answer) {
    Id.Answer value = Id.Answer.Yes;
    value.id();
  }
}
