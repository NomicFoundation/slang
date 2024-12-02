enum Answer {
  Yes,
  No
}

function id(Answer ans) returns (Answer) {
  return ans;
}

contract Test {
  using { id } for Answer;

  function testFunc() {
    string name = type(Test).name;
    Answer first = type(Answer).min.id();
  }
}
