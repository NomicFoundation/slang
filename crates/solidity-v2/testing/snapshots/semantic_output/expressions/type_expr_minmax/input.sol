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

    function testFunc() {
        // min/max should bind in >= 0.8.8
        type(Answer).min.id();
        type(Answer).max.id();
    }
}
