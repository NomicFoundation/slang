contract Test {
    enum Answer { Yes, No }

    function setAnswer(Answer ans) public {
    }

    function getAnswer() public returns (Answer ans) {
        ans = Answer.Yes;
    }

    function getOtherAnswer() public returns (Answer) {
        return Answer.No;
    }
}
