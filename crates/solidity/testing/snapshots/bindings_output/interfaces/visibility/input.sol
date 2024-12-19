interface Example {
    enum Choice { One, Two }
    struct Book { string author; }
    function calculate() external returns (int);
    function get_book() external returns (Book memory);
}

contract Test {
    Example.Choice choice = Example.Choice.One;
    Example.Book book;

    function test(Example e) public returns (int) {
        return e.calculate();
    }
}
