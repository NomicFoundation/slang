enum Answer {
  Yes
}

library Id {
  function id(Answer ans) returns (Answer) {
    return ans;
  }
}

contract Test {
  using Id for Answer;

  function testFunc() returns (Answer) {
    Answer value = Answer.Yes;
    value.id();
  }
}
