library Id {
  enum Answer {
    Yes
  }

  function id(Answer ans) returns (Answer) {
    return ans;
  }
}

contract Test {
  using Id for Id.Answer;

  function testFunc() returns (Id.Answer) {
    Id.Answer value = Id.Answer.Yes;
    value.id();
  }
}
