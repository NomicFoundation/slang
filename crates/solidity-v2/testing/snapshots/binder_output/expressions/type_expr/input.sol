contract Test {
    enum Answer { Yes, No }
    function testFunc() {
        string name = type(Test).name;
        Answer first = type(Answer).min;
    }
}
