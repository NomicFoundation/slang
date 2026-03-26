contract Test {
    event Log(
        string name,
        uint level
    );

    function test() public {
        emit Log({level: 1, name: "Testing"});
    }
}
