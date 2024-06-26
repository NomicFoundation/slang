contract Test {
    enum Answer { Yes, No }

    function setAnswer(Answer ans) {
    }

    function getAnswer() returns (Answer ans) {
        ans = Answer.Yes;
    }

    function getOtherAnswer() returns (Answer) {
        return Answer.No;
    }
}
