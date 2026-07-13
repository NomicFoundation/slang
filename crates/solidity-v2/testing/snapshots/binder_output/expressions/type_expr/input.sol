contract Test {
    enum Answer { Yes, No }
    function testFunc() public {
        string name = type(Test).name;
        Answer first = type(Answer).min;
    }
}
